use embedded_io::{Read, Write};
use heapless::Vec;

use crate::monitor_message::{MainToMonitor, MonitorToMain};

// look into this blog: https://ferrous-systems.com/blog/test-embedded-app/

pub struct MonitorSerialPort<const N: usize, TX, RX>
where
    TX: Write,
    RX: Read,
{
    buffer: Vec<u8, N>,
    to_read: Option<usize>,
    tx: TX,
    rx: RX,
}

impl<const N: usize, TX, RX> MonitorSerialPort<N, TX, RX>
where
    TX: Write,
    RX: Read,
{
    pub fn new(tx: TX, rx: RX) -> Self {
        MonitorSerialPort {
            buffer: Vec::new(),
            to_read: None,
            tx,
            rx,
        }
    }

    pub fn poll(&mut self) -> Option<MainToMonitor> {
        None
    }

    pub fn send(&mut self, msg: &MonitorToMain) -> Result<(), postcard::Error> {
        let bytes: Vec<u8, N> = postcard::to_vec(msg)?;

        for byte in bytes {
            nb::block!(self.tx.write(buf))
        }

        Ok(())
    }
}
