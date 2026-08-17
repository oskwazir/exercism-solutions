use std::fmt;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_DAY: i32 = 1440;

fn get_remainder(x: i32, n: i32) -> i32 {
    ((x % n) + n) % n
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes: i32 = (hours * MINUTES_IN_HOUR) + minutes;
        let normalized = get_remainder(total_minutes, MINUTES_IN_DAY);

        Clock {
            hours: normalized / MINUTES_IN_HOUR,
            minutes: get_remainder(normalized, MINUTES_IN_HOUR),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes: i32 = ((self.hours * MINUTES_IN_HOUR) + self.minutes) + minutes;
        let normalized = get_remainder(total_minutes, MINUTES_IN_DAY);

        Clock {
            hours: normalized / MINUTES_IN_HOUR,
            minutes: get_remainder(normalized, MINUTES_IN_HOUR),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
