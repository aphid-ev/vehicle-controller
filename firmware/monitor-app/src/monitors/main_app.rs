use common::timeout::Timeout;

use super::MonitorError;

pub struct MainAppMonitor {
    timout: Timeout,
    latest_ping: u32,
}

impl MainAppMonitor {
    pub fn new(timeout: usize) -> Self {
        MainAppMonitor {
            timout: Timeout::new(timeout),
            latest_ping: 0,
        }
    }

    pub fn send_ping(&mut self) {
        self.timout.reset();

        self.latest_ping += 1;

        // TODO: Send
    }

    pub fn tick(&mut self) -> Result<(), MonitorError> {
        self.timout.tick()?;

        // TODO: Check for pong

        Ok(())
    }
}
