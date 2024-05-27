use super::MonitorError;
use common::{
    throttle::Throttle,
    timeout::{Timeout, TimeoutError},
};

pub struct ThrottleMonitor<'a> {
    throttle: &'a Throttle,
    timeout: Timeout,
}

impl<'a> ThrottleMonitor<'a> {
    pub fn new(throttle: &'a Throttle, timeout: usize) -> Self {
        Self {
            throttle,
            timeout: Timeout::new(timeout),
        }
    }

    pub fn check(&mut self, sensor1: u16, sensor2: u16) -> Result<u16, MonitorError> {
        match self.throttle.position(sensor1, sensor2) {
            Err(err) => match self.timeout.tick() {
                Err(TimeoutError::Elapsed) => Err(MonitorError::Throttle(err)),
                Ok(_) => Ok(0),
            },
            Ok(position) => {
                self.timeout.reset();
                Ok(position)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::throttle::ThrottleError;

    #[test]
    fn monitor() {
        let throttle = Throttle::new((1000, 2000), (3000, 4000), 1500);
        let mut throttle_monitor = ThrottleMonitor::new(&throttle, 2);

        assert_eq!(throttle_monitor.check(1200, 3700), Ok(0));
        assert_eq!(throttle_monitor.check(1200, 3700), Ok(0));
        assert_eq!(
            throttle_monitor.check(1200, 3700),
            Err(MonitorError::ThrottleError(ThrottleError::SensorMismatch))
        );

        assert_eq!(throttle_monitor.check(1500, 3500), Ok(32767));
    }
}
