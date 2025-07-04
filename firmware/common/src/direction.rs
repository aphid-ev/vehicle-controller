use embassy_time::Duration;

use crate::input::{CriticalInputs, DirectionInput};

#[derive(Debug, PartialEq, Eq)]
pub enum DirectionError {
    Overspeed,
    IncosistentInput,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Direction {
    Forward,
    #[default]
    Neutral,
    Reverse,
    Error,
}

#[derive(Debug, Default)]
pub struct DirectionController {
    current: Direction,
}

impl DirectionController {
    const MAX_SHIFTING_SPEED: f32 = 3.0; // m/s
    const MAX_MISSMATCH_DURATION: Duration = Duration::from_millis(200);

    pub fn current(&self) -> Direction {
        self.current
    }

    pub fn tick(&mut self, speed: f32, input: CriticalInputs) -> Result<Direction, DirectionError> {
        if let Some(input) = input.direction() {
            if speed > Self::MAX_SHIFTING_SPEED {
                return Err(DirectionError::Overspeed);
            }

            self.current = match input {
                DirectionInput::Forward => Direction::Forward,
                DirectionInput::Reverse => Direction::Reverse,
                DirectionInput::Contradicting => return Err(DirectionError::IncosistentInput),
            }
        }

        Ok(self.current)
    }
}
