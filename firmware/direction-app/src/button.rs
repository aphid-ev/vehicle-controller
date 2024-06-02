pub struct ButtonFilter<const LIMIT: u32> {
    counter: u32,
    value: bool,
}

impl<const LIMIT: u32> ButtonFilter<LIMIT> {
    pub fn new(value: bool) -> Self {
        Self {
            counter: if value { LIMIT } else { 0 },
            value,
        }
    }

    pub fn sample(&mut self, value: bool) -> bool {
        (self.value, self.counter) = match (self.value, value) {
            (false, false) => (false, 0),
            (true, true) => (true, LIMIT),
            (false, true) => {
                self.counter += 1;
                let value = if self.counter >= LIMIT {
                    true
                } else {
                    self.value
                };
                (value, self.counter)
            }
            (true, false) => {
                self.counter -= 1;
                let value = if self.counter == 0 { false } else { self.value };
                (value, self.counter)
            }
        };

        self.value
    }
}
