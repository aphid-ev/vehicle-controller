
#[derive(Debug, PartialEq)]
pub enum TimeoutError {
    Elapsed
}


pub struct Timeout {
    reset: usize,
    counter: usize,
}

impl Timeout {
    pub fn new(reset: usize) -> Self {
        Timeout {
            reset,
            counter: reset,
        }
    }

    pub fn tick(&mut self) -> Result<(), TimeoutError> {
        self.counter = self.counter.checked_sub(1).ok_or(TimeoutError::Elapsed)?;
        Ok(())
    }

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