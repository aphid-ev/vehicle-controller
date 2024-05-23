#![no_std]
#![no_main]

mod board;

use board::EvCanCommand;
use common::monitor_message::*;
use cortex_m_rt::entry;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f0xx_hal::{can::bxcan::filter::Mask32, pac, prelude::*, timers::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut board = board::Board::new(dp);

    board.enable_can_bank(0, Mask32::accept_all());

    let mut led_enabled = false;

    let mut timer = Timer::syst(cp.SYST, 1000.hz(), &board.rcc);

    let msg = MonitorToMain {
        ping: 12345,
        state: MonitorState::Operational,
    };

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

        board.serial_send(&msg);

        nb::block!(timer.wait()).unwrap();
    }
}
