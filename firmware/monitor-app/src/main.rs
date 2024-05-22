#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use stm32f0xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.configure().sysclk(8.mhz()).freeze(&mut dp.FLASH);
    let gpioa = dp.GPIOA.split(&mut rcc);

    let mut led = cortex_m::interrupt::free(|cs| gpioa.pa1.into_push_pull_output(cs));

    loop {
        // Turn PA1 on a million times in a row
        for _ in 0..1_000_000 {
            led.set_high().ok();
        }
        // Then turn PA1 off a million times in a row
        for _ in 0..1_000_000 {
            led.set_low().ok();
        }
    }
}
