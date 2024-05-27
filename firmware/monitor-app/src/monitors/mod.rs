use common::{throttle::ThrottleError, timeout::TimeoutError};

#[derive(Debug, PartialEq)]
pub enum MonitorError {
    Throttle(ThrottleError),
    Torque,
    Main,
    Timout,
}

impl From<TimeoutError> for MonitorError {
    fn from(_value: TimeoutError) -> Self {
        MonitorError::Timout
    }
}

mod main_app;
pub use main_app::*;

mod throttle;
pub use throttle::*;

mod torque;
pub use torque::*;
