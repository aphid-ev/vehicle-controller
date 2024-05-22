#![no_std]
#![no_main]

mod board;

use board::EvCanCommand;
use cortex_m_rt::entry;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f0xx_hal::{pac, prelude::*, timers::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut board = board::Board::new(dp);

    let mut led_enabled = false;

    let mut timer = Timer::syst(cp.SYST, 1000.hz(), &board.rcc);
    loop {
        if board.is_button_pressed() {
            board.ev_can_send(EvCanCommand::SetLed(led_enabled)).ok();
            led_enabled = !led_enabled;

            // wait until the button is released
            while board.is_button_pressed() {
                cortex_m::asm::nop();
            }
        }

        match board.ev_can_receive() {
            Some(EvCanCommand::SetLed(true)) => board.enable_led(),
            Some(EvCanCommand::SetLed(false)) => board.disable_led(),
            _ => {}
        }

        nb::block!(timer.wait()).unwrap();
    }
}
