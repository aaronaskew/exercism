use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_overflow = match (minutes, minutes % 60) {
            (0.., _) => minutes / 60,
            (..=-1, 0) => minutes / 60,
            (..=-1, _) => minutes / 60 - 1,
        };

        let total_hours = hours + hours_overflow;

        Self {
            hours: match (total_hours, total_hours % 24) {
                (_, 0) => 0,
                (0.., _) => total_hours as u8 % 24, 
                (..=-1, _) => (24 + total_hours % 24) as u8,
            },
            minutes: match (minutes, minutes % 60) {
                (_, 0) => 0,
                (0.., _) => (minutes % 60) as u8,
                (..=-1, _) => (60 + (minutes % 60)) as u8,
            },
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours as i32, self.minutes as i32 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
