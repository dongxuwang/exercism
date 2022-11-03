
use std::fmt;

const HOUR: i32 = 60;
const DAY: i32 = HOUR * 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.minutes.div_euclid(HOUR), self.minutes.rem_euclid(HOUR))
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{ minutes: (hours * HOUR + minutes).rem_euclid(DAY)}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}
