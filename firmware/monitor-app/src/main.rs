#![no_std]
#![no_main]

mod board;
mod monitors;

use common::{monitor_message::*, throttle::Throttle};
use cortex_m_rt::entry;
use monitors::{MainAppMonitor, ThrottleMonitor, TorqueMonitor};
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

    let throttle = Throttle::new((1000, 2000), (3000, 4000), 1500);
    let mut throttle_monitor = ThrottleMonitor::new(&throttle, 10);
    let mut torque_monitor = TorqueMonitor::new(10, 10);
    let mut main_app_monitor = MainAppMonitor::new(10);

    loop {
        if board.is_button_pressed() {
            board.ev_can_send(EvCanCommand::SetLed(led_enabled)).ok();
            led_enabled = !led_enabled;

            // wait until the button is released
            while board.is_button_pressed() {
                cortex_m::asm::nop();
            }
        }

        board.serial_send(&msg);

        // Monitor main application
        main_app_monitor.tick().ok(); // TODO: Manage errors.

        // Monitor acceleration pedal
        let (acc_sensor1, acc_sensor2) = board.read_throttle_sensors();
        let throttle_position = match throttle_monitor.check(acc_sensor1, acc_sensor2) {
            Err(_err) => {
                // TODO: goto safe state
                0
            }
            Ok(pos) => pos,
        };

        // Monitor torque request
        torque_monitor.tick().ok(); // TODO: manage error
        if let Some(frame) = board.ev_can_receive() {
            torque_monitor.frame(throttle_position, &frame).ok(); // TODO: Manage error
        };

        nb::block!(timer.wait()).unwrap();
    }
}
