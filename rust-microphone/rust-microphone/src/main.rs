#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::interrupt::Mutex;
use microbit::{
    Board,
    hal::{
        gpio::{Level, OpenDrainConfig},
        saadc::SaadcConfig,
        Saadc,
        gpiote::Gpiote,
    },
    pac::{self, interrupt},
};

use core::cell::RefCell;
static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static SAADC: Mutex<RefCell<Option<Saadc>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let gpiote = Gpiote::new(board.GPIOTE);

        let channel0 = gpiote.channel0();
        channel0
            .input_pin(&board.buttons.button_a.degrade())
            .hi_to_lo()
            .enable_interrupt();
        channel0.reset_events();

        let channel1 = gpiote.channel1();
        channel1
            .input_pin(&board.buttons.button_b.degrade())
            .hi_to_lo()
            .enable_interrupt();
        channel1.reset_events();

        cortex_m::interrupt::free(move |cs| {

            let saadc_config = SaadcConfig::default();
            let saadc = Saadc::new(board.ADC, saadc_config);
            let mic_in = board.microphone_pins.mic_in.into_floating_input();

            board
                .microphone_pins
                .mic_run
                .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);

            /* Enable external GPIO interrupts */
            unsafe {
                pac::NVIC::unmask(pac::Interrupt::GPIOTE);
            }
            pac::NVIC::unpend(pac::Interrupt::GPIOTE);
            *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

            defmt::info!("Welcome to the buttons demo. Press buttons A and/or B for some action.");
        });

        loop {
            continue;
        }
    }
    panic!();
}

#[interrupt]
fn GPIOTE() {
        cortex_m::interrupt::free(|cs| {
            if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
                let buttonapressed = gpiote.channel0().is_event_triggered();
                let buttonbpressed = gpiote.channel1().is_event_triggered();

                if let Some(saadc) = SAADC.borrow(cs).borrow().as_ref() {

                    match (buttonapressed, buttonbpressed) {
                        (false, false) => todo!(),
                        (true, false) | (false, true) | (true, true) => todo!(),
                    }
                    /* Clear events */
                    gpiote.channel0().reset_events();
                    gpiote.channel1().reset_events();
                }
            }
        });
}

/*fn measure_and_print() {
        let mic_value = saadc
            .read_channel(&mut mic_in)
            .expect("could not read value of microphone") as i32;
        println!("mic value: {}", mic_value);
}
*/