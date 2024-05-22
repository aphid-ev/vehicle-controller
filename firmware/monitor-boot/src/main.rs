#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f0xx_hal::{
    can::{
        bxcan::{Can, Frame, StandardId},
        CanInstance,
    },
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.configure().sysclk(8.mhz()).freeze(&mut dp.FLASH);
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut led = cortex_m::interrupt::free(|cs| gpioa.pa1.into_push_pull_output(cs));

    let can_rx = cortex_m::interrupt::free(|cs| gpiob.pb8.into_alternate_af4(cs));
    let can_tx = cortex_m::interrupt::free(|cs| gpiob.pb9.into_alternate_af4(cs));

    let can = CanInstance::new(dp.CAN, can_tx, can_rx, &mut rcc);
    let mut bxcan = Can::builder(can).set_bit_timing(0).enable();

    let frame = Frame::new_data(StandardId::new(0x123).unwrap(), [0u8; 8]);

    loop {
        // Turn PA1 on a million times in a row
        for _ in 0..1_000_000 {
            led.set_high().ok();
        }
        // Then turn PA1 off a million times in a row
        for _ in 0..1_000_000 {
            led.set_low().ok();
        }
        bxcan.transmit(&frame).unwrap();
    }
}
