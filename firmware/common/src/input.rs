use core::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
pub enum DirectionInput {
    Forward,
    Reverse,
    Contradicting,
}

#[derive(Debug, PartialEq)]
pub enum ThrottleInput {
    Invalid,
    Valid(f32),
}

#[derive(Debug, PartialEq)]
pub struct CriticalInputs {
    forward: bool,
    reverse: bool,
    brake: bool,
    throttle1: f32,
    throttle2: f32,
}

impl CriticalInputs {
    pub fn new(forward: bool, reverse: bool, brake: bool, throttle1: f32, throttle2: f32) -> Self {
        Self {
            forward,
            reverse,
            brake,
            throttle1,
            throttle2,
        }
    }

    pub fn direction(&self) -> Option<DirectionInput> {
        match (self.forward, self.reverse) {
            (false, false) => None,
            (true, false) => Some(DirectionInput::Forward),
            (false, true) => Some(DirectionInput::Reverse),
            (true, true) => Some(DirectionInput::Contradicting),
        }
    }

    pub fn brake(&self) -> bool {
        self.brake
    }

    pub fn throttle(&self) -> ThrottleInput {
        const T_POS: Line = Line::new(1.0, 0.0);
        const T_NEG: Line = Line::new(-1.0, 1.0);
        const MAX_THROTTLE_DEVIATION: f32 = 0.01;
        const VALID: RangeInclusive<f32> = 0.0..=1.0;

        let pos = T_POS.f(self.throttle1);
        let neg = T_NEG.f(self.throttle2);

        // Validate
        if !(VALID.contains(&pos) && VALID.contains(&neg))
            && (pos - neg).abs() < MAX_THROTTLE_DEVIATION
        {
            let mean = (pos + neg) / 2.0;
            return ThrottleInput::Valid(mean);
        }
        ThrottleInput::Invalid
    }
}

struct Line {
    k: f32,
    m: f32,
}

impl Line {
    const fn new(k: f32, m: f32) -> Self {
        Self { k, m }
    }

    fn f(&self, x: f32) -> f32 {
        self.k * x + self.m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction() {
        let input = CriticalInputs::new(false, false, false, 0.0, 0.0);

        assert_eq!(input.direction(), None);
    }

    #[test]
    fn brake() {}

    #[test]
    fn throttle() {}
}
