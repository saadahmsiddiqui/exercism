use std::{fmt, cmp};

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}",
            self.hours,
            self.minutes
        )
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}",
            self.hours,
            self.minutes
        )
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let start_hours = match hours > 0 {
            true => 0,
            false => 24
        };

        let mut clock_hours = start_hours;
        let mut clock_hours = if hours > 0 {
            let mut hour_counter = hours;

            while hour_counter > 0 {
                if clock_hours == 24 { clock_hours = 0; } else { clock_hours += 1; }
                hour_counter -= 1;
            }

            clock_hours
        } else {
            let mut hour_counter = hours.abs();

            while hour_counter > 0 {
                if clock_hours == 0 { clock_hours = 24; } else { clock_hours -= 1; }
                hour_counter -=1;
            }

            clock_hours
        };

        let clock_minutes = if minutes > 0 {
            if minutes > 59 {
                let mut hours_to_add = minutes / 60;

                while hours_to_add > 0 {
                    if clock_hours == 24 { clock_hours = 0; } else { clock_hours += 1; }
                    hours_to_add -= 1;
                }

                minutes % 60
            } else {
                minutes
            }
        } else {
            let minutes_to_remove = minutes.abs();

            if minutes_to_remove > 59 {
                let mut hours_to_remove = minutes_to_remove / 60;

                while hours_to_remove > 0 {
                    if clock_hours == 0 { clock_hours = 24; } else { clock_hours -= 1; }
                    hours_to_remove -= 1;
                }

                60 - (minutes_to_remove % 60)                
            } else {
                clock_hours = clock_hours - 1;
                60 - minutes_to_remove
            }
        };

        Clock { hours: clock_hours, minutes: clock_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
