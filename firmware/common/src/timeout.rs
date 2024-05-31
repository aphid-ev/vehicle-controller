
#[derive(Debug, PartialEq)]
pub enum TimeoutError {
    Elapsed
}

/// Helper for managing unit based timeouts. The caller is responsible to call
/// `tick()` with a suitable periodicity.
/// # Example
/// ```
/// use common::timeout::{Timeout, TimeoutError};
/// let mut timeout = Timeout::new(2);
/// assert_eq!(timeout.tick(), Ok(()));
/// assert_eq!(timeout.tick(), Ok(()));
/// assert_eq!(timeout.tick(), Err(TimeoutError::Elapsed));
/// ```
pub struct Timeout {
    reset: usize,
    counter: usize,
}

impl Timeout {
    /// Create a new timeout object with a given reset value.
    pub fn new(reset: usize) -> Self {
        Timeout {
            reset,
            counter: reset,
        }
    }

    /// Tick count down one unit, will return `Err(TimeoutError::Elapsed)` if
    /// underflow otherwise `Ok(())`
    pub fn tick(&mut self) -> Result<(), TimeoutError> {
        self.counter = self.counter.checked_sub(1).ok_or(TimeoutError::Elapsed)?;
        Ok(())
    }

    /// Reset counter to the reset value
    pub fn reset(&mut self) {
        self.counter = self.reset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timout() {
        let mut timeout = Timeout::new(2);

        assert_eq!(timeout.tick(), Ok(()));
        assert_eq!(timeout.tick(), Ok(()));
        assert_eq!(timeout.tick(), Err(TimeoutError::Elapsed));
        assert_eq!(timeout.tick(), Err(TimeoutError::Elapsed));

        timeout.reset();
        assert_eq!(timeout.tick(), Ok(()));
        assert_eq!(timeout.tick(), Ok(()));
        assert_eq!(timeout.tick(), Err(TimeoutError::Elapsed));
    }
}
