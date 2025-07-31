use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h = 60 * hours.rem_euclid(24);
        let m = minutes.rem_euclid(1440);
        Clock { minutes: (m + h) % 1440 }
    }

    fn convert(&self) -> (i32, i32) {
        (self.minutes / 60, self.minutes % 60)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = self.convert();
        Self::new(h, m + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (h, m) = self.convert();
        write!(f, "{:02}:{:02}", h, m)
    }
}
