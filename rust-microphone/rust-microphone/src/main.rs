#![no_std]
#![no_main]


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

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {

        let saadc_config = SaadcConfig::default();
        let mut  saadc = Saadc::new(board.ADC, saadc_config);
        let mut mic_in = board.microphone_pins.mic_in.into_floating_input();

        board
            .microphone_pins
            .mic_run
            .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);

        loop {
            let mic_value = saadc
                .read_channel(&mut mic_in)
                .expect("could not read value of microphone") as i32;
            println!("mic value: {}", mic_value);
        }
    }
    panic!("End");
}

/*fn measure_and_print() {
        let mic_value = saadc
            .read_channel(&mut mic_in)
            .expect("could not read value of microphone") as i32;
        println!("mic value: {}", mic_value);
}
*/