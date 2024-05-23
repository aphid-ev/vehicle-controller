#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    rcc.cfgr.use_hse(8.MHz()).freeze();


    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa0.into_push_pull_output();

    loop {
        led.toggle();

        for _ in 0..10000 {
            cortex_m::asm::nop();
        }
    }
}
