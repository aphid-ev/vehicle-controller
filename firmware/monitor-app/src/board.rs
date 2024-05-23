use core::convert::Infallible;

use stm32f0xx_hal::{
    can::{
        bxcan::{filter::Mask32, Can, Data, Frame, Id, StandardId, TransmitStatus},
        CanInstance,
    },
    gpio::{
        gpioa::PA5,
        gpiob::{PB8, PB9},
        gpioc::PC13,
        Alternate, GpioExt, Input, Output, PullUp, PushPull, AF4,
    },
    pac,
    prelude::*,
    rcc::{HSEBypassMode, Rcc},
};

pub enum EvCanCommand {
    SetLed(bool),
}

impl EvCanCommand {
    fn id(&self) -> Id {
        match self {
            EvCanCommand::SetLed(_) => StandardId::new(0x123).unwrap().into(),
        }
    }

    fn data(&self) -> Data {
        match self {
            EvCanCommand::SetLed(state) => {
                if *state {
                    [1u8].into()
                } else {
                    [0u8].into()
                }
            }
        }
    }

    fn from_frame(frame: Frame) -> Result<Self, ()> {
        if frame.id() == StandardId::new(0x123).unwrap().into() {
            match frame.data().unwrap()[0] {
                0 => Ok(EvCanCommand::SetLed(false)),
                1 => Ok(EvCanCommand::SetLed(true)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

pub struct Board {
    pub led: PA5<Output<PushPull>>,
    pub button: PC13<Input<PullUp>>,
    pub ev_can: Can<CanInstance<PB9<Alternate<AF4>>, PB8<Alternate<AF4>>>>,
    pub rcc: Rcc,
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

        let (led, button, can_rx, can_tx) = cortex_m::interrupt::free(|cs| {
            let led = gpioa.pa5.into_push_pull_output(cs);
            let button = gpioc.pc13.into_pull_up_input(cs);

            let can_rx = gpiob.pb8.into_alternate_af4(cs);
            let can_tx = gpiob.pb9.into_alternate_af4(cs);

            (led, button, can_rx, can_tx)
        });

        let can = CanInstance::new(dp.CAN, can_tx, can_rx, &mut rcc);
        let mut ev_can = Can::builder(can)
            .set_bit_timing(0x001c0005)
            .set_loopback(true)
            .enable();

        let mut filters = ev_can.modify_filters();
        filters.enable_bank(0, Mask32::accept_all());
        drop(filters);

        Self {
            led,
            button,
            ev_can,
            rcc,
        }
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

    pub fn ev_can_send(&mut self, command: EvCanCommand) -> nb::Result<TransmitStatus, Infallible> {
        let tx_frame = Frame::new_data(command.id(), command.data());
        self.ev_can.transmit(&tx_frame)
    }

    pub fn ev_can_receive(&mut self) -> Option<EvCanCommand> {
        if let Ok(rx_frame) = self.ev_can.receive() {
            EvCanCommand::from_frame(rx_frame).ok()
        } else {
            None
        }
    }
}
