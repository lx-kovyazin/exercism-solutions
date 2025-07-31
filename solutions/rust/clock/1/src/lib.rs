use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h = 60 * match hours % 24 {
            h if h < 0 => h + 24,
            h => h,
        };
        let m = match minutes % 1440 {
            m if m < 1440 => m + 1440,
            m => m
        };
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
