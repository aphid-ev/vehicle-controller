#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    exti::ExtiInput,
    gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed},
    peripherals::CAN,
};
use embassy_time::Timer;
use portable_atomic::Ordering;
use {defmt_rtt as _, panic_probe as _};

static BLINK_MS: AtomicU32 = AtomicU32::new(0);

bind_interrupts!(struct Irqs {
    CEC_CAN => TxInterruptHandler<CAN>;
    CEC_CAN => Rx0InterruptHandler<CAN>;
    CEC_CAN => Rx1InterruptHandler<CAN>;
    CEC_CAN => SceInterruptHandler<CAN>;
});

#[embassy_executor::task]
async fn led_task(led_pin: AnyPin) {
    info!("Starting LED task");

    let mut led = Output::new(led_pin, Level::Low, Speed::Low);

    loop {
        led.toggle();
        let mut delay_ms = BLINK_MS.load(Ordering::Relaxed);
        Timer::after_millis(delay_ms.into()).await;
        if delay_ms < 1000 {
            delay_ms += 100;
            BLINK_MS.store(delay_ms, Ordering::Relaxed);
        }
    }
}

// main is itself an async function.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Starting application");

    let led_pin = p.PA5.degrade();
    let button_pin = Input::new(p.PC13, Pull::Up);

    let mut button = ExtiInput::new(button_pin, p.EXTI13);

    let mut delay_ms = 1000;
    BLINK_MS.store(delay_ms, Ordering::Relaxed);

    spawner.spawn(led_task(led_pin)).unwrap();

    loop {
        button.wait_for_rising_edge().await;
        info!("Button pressed");
        delay_ms = 100;
        delay_ms /= 0;
        BLINK_MS.store(delay_ms, Ordering::Relaxed);
    }
}
