#![no_std]
#![no_main]

use button::ButtonFilter;
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{
        self,
        filter::{StandardFilter, StandardFilterSlot},
        Frame, OperatingMode,
    },
    gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed},
    peripherals::FDCAN1,
    time::Hertz,
    Config,
};
use embassy_time::Timer;

use defmt_rtt as _;
use panic_probe as _;

mod button;

static FORWARD: AtomicBool = AtomicBool::new(false);
static NEUTRAL: AtomicBool = AtomicBool::new(false);
static REVERSE: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
async fn button_task(forward_pin: AnyPin, reverse_pin: AnyPin, neutral_pin: AnyPin) {
    info!("Button task started");

    let forward_button = Input::new(forward_pin, Pull::Up);
    let neutral_button = Input::new(neutral_pin, Pull::Up);
    let reverse_button = Input::new(reverse_pin, Pull::Up);

    let mut forward_filter = ButtonFilter::<3>::new(false);
    let mut neutral_filter = ButtonFilter::<3>::new(false);
    let mut reverse_filter = ButtonFilter::<3>::new(false);

    loop {
        FORWARD.store(
            forward_filter.sample(forward_button.is_low()),
            Ordering::Relaxed,
        );
        NEUTRAL.store(
            neutral_filter.sample(neutral_button.is_low()),
            Ordering::Relaxed,
        );
        REVERSE.store(
            reverse_filter.sample(reverse_button.is_low()),
            Ordering::Relaxed,
        );
        Timer::after_millis(10).await;
    }
}

bind_interrupts!(struct Irqs {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(16_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL85,
            divp: None,
            divq: Some(PllQDiv::DIV8), // 42.5 Mhz for fdcan.
            divr: Some(PllRDiv::DIV2), // Main system clock at 170 MHz
        });
        config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
        config.rcc.sys = Sysclk::PLL1_R;
        config.enable_debug_during_sleep = true;
    }
    let p = embassy_stm32::init(config);

    let mut forward_led = Output::new(p.PA0, Level::Low, Speed::Low);
    let mut neutral_led = Output::new(p.PA1, Level::Low, Speed::Low);
    let mut reverse_led = Output::new(p.PA2, Level::Low, Speed::Low);
    let forward_pin = p.PA3.degrade();
    let neutral_pin = p.PA4.degrade();
    let reverse_pin = p.PA5.degrade();

    let mut can = can::CanConfigurator::new(p.FDCAN1, p.PB8, p.PB9, Irqs);
    can.properties().set_standard_filter(
        StandardFilterSlot::_0,
        StandardFilter::accept_all_into_fifo0(),
    );
    can.set_bitrate(500_000);
    let mut can = can.start(OperatingMode::NormalOperationMode);

    spawner
        .spawn(button_task(forward_pin, reverse_pin, neutral_pin))
        .unwrap();

    loop {
        let forward = FORWARD.load(Ordering::Relaxed);
        let _neutral = NEUTRAL.load(Ordering::Relaxed);
        let _reverse = REVERSE.load(Ordering::Relaxed);

        if forward {
            forward_led.set_high();
            neutral_led.set_high();
            reverse_led.set_high();
            info!("Sending frame");
            let frame = Frame::new_standard(0x123, &[1u8; 8]).unwrap();
            can.write(&frame).await;
        } else {
            forward_led.set_low();
            neutral_led.set_low();
            reverse_led.set_low();
        }
        Timer::after_millis(100).await;
    }
}
