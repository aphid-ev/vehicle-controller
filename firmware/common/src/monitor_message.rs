use serde::{Deserialize, Serialize};

/// The size of buffeer needed to recieve a full struct, should be
/// big enough to hold a worst case postcard serialization of both structs
pub const MONITOR_MESSAGE_BUFFER_SIZE: usize = 32;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MonitorError {
    PingError,
    AcceleratorError,
    TorqueRequestError,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MainError {
    AcceleratorError
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MonitorState {
    Operational,
    Error(MonitorError),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MainState {
    Operational,
    Error(MainError),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorToMain {
    pub ping: u64,
    pub state: MonitorState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainToMonitor {
    pub pong: u64,
    pub state: MainState,
    pub accelerator: u16,
    pub high_side_on: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_to_main() {
        let msg = MonitorToMain {
            ping: 12345,
            state: MonitorState::Error(MonitorError::PingError),
        };

        let mut buf = [0u8; MONITOR_MESSAGE_BUFFER_SIZE];

        let bytes = postcard::to_slice(&msg, &mut buf).unwrap();

        let msg_out: MonitorToMain = postcard::from_bytes(bytes).unwrap();

        assert_eq!(msg.ping, msg_out.ping);
        assert_eq!(msg.state, msg_out.state);
    }

    #[test]
    fn main_to_monitor() {
        let msg = MainToMonitor {
            pong: 12345,
            state: MainState::Error(
                MainError::AcceleratorError
            ),
            accelerator: 54321,
            high_side_on: true,
        };

        let mut buf = [0u8; MONITOR_MESSAGE_BUFFER_SIZE];

        let bytes = postcard::to_slice(&msg, &mut buf).unwrap();

        let msg_out: MainToMonitor = postcard::from_bytes(bytes).unwrap();

        assert_eq!(msg.pong, msg_out.pong);
        assert_eq!(msg.state, msg_out.state);
        assert_eq!(msg.accelerator, msg_out.accelerator);
        assert_eq!(msg.high_side_on, msg_out.high_side_on);
    }
}