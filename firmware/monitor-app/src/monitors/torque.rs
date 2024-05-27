use common::{ev_can::EvCanFrame, timeout::Timeout};

use super::MonitorError;

pub struct TorqueMonitor {
    frame_timeout: Timeout,
    error_timeout: Timeout,
}

impl TorqueMonitor {
    pub fn new(frame_timeout: usize, error_timeout: usize) -> Self {
        Self {
            frame_timeout: Timeout::new(frame_timeout),
            error_timeout: Timeout::new(error_timeout),
        }
    }

    pub fn tick(&mut self) -> Result<(), MonitorError> {
        self.frame_timeout.tick()?;
        Ok(())
    }

    pub fn frame(&mut self, acc_position: u16, frame: &EvCanFrame) -> Result<(), MonitorError> {
        if let EvCanFrame::TorqueRequest { torque, counter } = frame {
            // TODO: Check that frame corresponds with accelerator position.
            Ok(())
        } else {
            Err(MonitorError::Torque)
        }
    }
}
