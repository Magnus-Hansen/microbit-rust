#![no_std]
#![no_main]


use defmt_rtt as _;
use panic_halt as _;
use core::fmt::Write;
use embedded_io::Read;

use cortex_m_rt::entry;
use microbit::{
    Board,
    hal::{
        gpio::{Level, OpenDrainConfig},
        saadc::SaadcConfig,
        Saadc,
    },
};
use defmt::println;
use embedded_hal::delay::DelayNs;
use microbit::hal::Timer;

#[cfg(feature = "v2")]
use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let saadc_config = SaadcConfig::default();
        let mut saadc = Saadc::new(board.ADC, saadc_config);
        let mut mic_in = board.microphone_pins.mic_in.into_floating_input();

        board
            .microphone_pins
            .mic_run
            .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);

        #[cfg(feature = "v2")]
        let mut serial = {
            let serial = uarte::Uarte::new(
                board.UARTE0,
                board.uart.into(),
                Parity::EXCLUDED,
                Baudrate::BAUD115200,
            );
            UartePort::new(serial)
        };

        loop {
            timer.delay_ms(3000);
            let mic_value = saadc
                .read_channel(&mut mic_in)
                .expect("could not read value of microphone") as i32;
            println!("mic value: {}", mic_value);

            write!(serial, "{}\r\n", mic_value).unwrap();
        }
    }
    panic!("End!")
}
