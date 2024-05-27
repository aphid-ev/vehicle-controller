use core::num::TryFromIntError;


#[derive(Debug, PartialEq)]
pub enum ThrottleError {
    SensorOutOfRange,
    SensorMismatch,
    IntegerError(TryFromIntError),
}


impl From<TryFromIntError> for ThrottleError {
    fn from(value: TryFromIntError) -> Self {
        Self::IntegerError(value)
    }
}

/// Maps a u16 range to the full 0-65535
struct Map {
    min: u16,
    max: u16,
}

impl Map {
    /// Create a new map for `min..=max` to `0..=u16::MAX`. 
    /// Make sure that `min < max`, otherwise shit may break.
    fn new(min: u16, max: u16) -> Self {
        Self { 
            min: min.min(max),
            max: max.max(min),
        }
    }

    /// Map a value from a specific range to the full `0..=u16::MAX` range
    fn map(&self, value: u16) -> Option<u16> {
        if value < self.min || value > self.max {
            return None
        }

        let mut map = value.checked_sub(self.min)? as u32;
        map *= u16::MAX as u32;
        map = map.checked_div(self.min.abs_diff(self.max) as u32)?;

        u16::try_from(map).ok()
    }
}

/// Struct for maping throttle input from two separate sensors into one value while also
/// verifying that they are within a tolerance. It will work with sensor values in millivolts
/// and throttle position as thee full u16 range (`0..=u16:MAX`).
/// 
/// # Example
/// ```
/// use common::throttle::Throttle;
/// // Sensor 1: 1.0 - 2.0 V
/// // Sensor 2: 3.0 - 4.0 V
/// let throttle = Throttle::new((1000,2000), (3000,4000), 250);
/// 
/// // Throttle input of 1.5 V and 3.5 V should result in 50% throttle -> u16::MAX * 50%
/// assert_eq!(throttle.position(1500, 3500), Ok(32767));
/// ```
pub struct Throttle {
    sensor1_map: Map,
    sensor2_map: Map,
    tolerance: u16,
}

impl Throttle {
    /// Create a new `Throttle` instance with values for both sensors and a tolerance. 
    /// 
    /// `sensor1: (u16, u16)` Should be a tuple with the min/max voltage in millivolts
    /// 
    /// `sensor2: (u16, u16)` Should be a tuple with the min/max voltage in millivolts
    /// 
    /// `tolerance: u16` should be the full range (`0..=u16::MAX`) tolerance between both sensors e.g. 655 -> 1%
    pub fn new(sensor1: (u16, u16), sensor2: (u16, u16), tolerance: u16) -> Self {
        Throttle { 
            sensor1_map: Map::new(
                sensor1.0, sensor1.1), 
            sensor2_map: Map::new(
                sensor2.0, sensor2.1), 
            tolerance, 
        }
    }

    /// get the checked throttle position based on two measured sensor voltages
    pub fn position(&self, sensor1: u16, sensor2: u16) -> Result<u16, ThrottleError> {
        let sensor1 = self.sensor1_map.map(sensor1).ok_or(ThrottleError::SensorOutOfRange)?;
        let sensor2 = self.sensor2_map.map(sensor2).ok_or(ThrottleError::SensorOutOfRange)?;

        if sensor1.abs_diff(sensor2) > self.tolerance {
            return Err(ThrottleError::SensorMismatch);
        }

        let mean = (sensor1 as u32 + sensor2 as u32) / 2;

        Ok(u16::try_from(mean)?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map() {
        let map = Map::new(1000, 2000);

        assert_eq!(map.map(999), None);
        assert_eq!(map.map(2001), None);
        assert_eq!(map.map(1000), Some(0));
        assert_eq!(map.map(1500), Some(32767));
        assert_eq!(map.map(2000), Some(65535));
    }

    #[test]
    fn throttle() {
        let throttle = Throttle::new((1000,2000), (3000,4000), 250);

        assert_eq!(throttle.position(1500, 3500), Ok(32767));
        assert_eq!(throttle.position(500, 3500), Err(ThrottleError::SensorOutOfRange));
        assert_eq!(throttle.position(1500, 5000), Err(ThrottleError::SensorOutOfRange));
        assert_eq!(throttle.position(1501, 3500), Ok(32800));
        assert_eq!(throttle.position(1505, 3500), Err(ThrottleError::SensorMismatch));
    }
}