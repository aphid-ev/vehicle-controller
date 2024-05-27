use core::convert::Infallible;

use common::{
    ev_can::EvCanFrame,
    monitor_message::{MonitorToMain, MONITOR_MESSAGE_BUFFER_SIZE},
};
use stm32f0xx_hal::{
    can::{
        bxcan::{filter::BankConfig, Can, Data, Frame, Id, StandardId, TransmitStatus},
        CanInstance,
    },
    gpio::{
        gpioa::{PA2, PA3, PA5},
        gpiob::{PB8, PB9},
        gpioc::PC13,
        Alternate, GpioExt, Input, Output, PullUp, PushPull, AF1, AF4,
    },
    pac::{self, USART2},
    prelude::*,
    rcc::{HSEBypassMode, Rcc},
    serial::Serial,
};

pub struct Board {
    pub led: PA5<Output<PushPull>>,
    pub button: PC13<Input<PullUp>>,
    pub ev_can: Can<CanInstance<PB9<Alternate<AF4>>, PB8<Alternate<AF4>>>>,
    pub rcc: Rcc,
    pub serial: Serial<USART2, PA2<Alternate<AF1>>, PA3<Alternate<AF1>>>,
}

impl Board {
    pub fn new(mut dp: pac::Peripherals) -> Self {
        let mut rcc = dp
            .RCC
            .configure()
            .hse(8.mhz(), HSEBypassMode::Bypassed)
            .sysclk(48.mhz())
            .pclk(48.mhz())
            .freeze(&mut dp.FLASH);

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);
        let gpioc = dp.GPIOC.split(&mut rcc);

        cortex_m::interrupt::free(|cs| {
            let led = gpioa.pa5.into_push_pull_output(cs);
            let button = gpioc.pc13.into_pull_up_input(cs);

            let can_rx = gpiob.pb8.into_alternate_af4(cs);
            let can_tx = gpiob.pb9.into_alternate_af4(cs);

            let serial_rx = gpioa.pa3.into_alternate_af1(cs);
            let serial_tx = gpioa.pa2.into_alternate_af1(cs);

            let can = CanInstance::new(dp.CAN, can_tx, can_rx, &mut rcc);
            let ev_can = Can::builder(can)
                .set_bit_timing(0x001c0005)
                .set_loopback(true)
                .enable();

            let serial = Serial::usart2(dp.USART2, (serial_tx, serial_rx), 115_200.bps(), &mut rcc);

            Self {
                led,
                button,
                ev_can,
                rcc,
                serial,
            }
        })
    }

    pub fn enable_led(&mut self) {
        self.led.set_high().ok();
    }

    pub fn disable_led(&mut self) {
        self.led.set_low().ok();
    }

    pub fn is_button_pressed(&self) -> bool {
        self.button.is_low().unwrap_or(false)
    }

    pub fn enable_can_bank(&mut self, index: u8, mask: impl Into<BankConfig>) {
        let mut filters = self.ev_can.modify_filters();
        filters.enable_bank(index, mask);
    }

    // TODO: This should not be needed once CAN is in silent mode
    pub fn ev_can_send(&mut self, frame: EvCanFrame) -> nb::Result<TransmitStatus, Infallible> {
        self.ev_can.transmit(&frame.into())
    }

    pub fn ev_can_receive(&mut self) -> Option<EvCanFrame> {
        if let Ok(rx_frame) = self.ev_can.receive() {
            Ok(EvCanFrame::try_from(rx_frame).ok()?)
        } else {
            None
        }
    }

    pub fn serial_send(&mut self, message: &MonitorToMain) {
        let bytes: heapless::Vec<u8, MONITOR_MESSAGE_BUFFER_SIZE> =
            postcard::to_vec(message).unwrap();

        for b in bytes {
            nb::block!(self.serial.write(b)).unwrap();
        }
    }

    /// Read the throttle sensor ADC inputs in millivolts
    pub fn read_throttle_sensors(&self) -> (u16, u16) {
        (1500, 3500)
    }
}
