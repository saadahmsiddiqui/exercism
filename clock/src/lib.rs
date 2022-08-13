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

fn add_hour_safe(current_hours: i32, hours: i32) -> i32 {
    match hours > 0 {
        true => (current_hours + hours) % 24,
        false => {
            let mut clock_hours = current_hours;
            let mut hours_to_remove = hours.abs();
            while hours_to_remove > 0 {
                clock_hours = if clock_hours == 0 { 24 } else { clock_hours - 1 };
                hours_to_remove -= 1;
            }

            clock_hours
        }
    }    
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock { hours: 0, minutes: 0 };
        clock = clock.add_hours(hours);
        clock = clock.add_minutes(minutes);
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut curr_hours = self.hours;
        let mut cl_minutes = self.minutes;

        if minutes > 0 {
            let remaining_minutes;

            if minutes > 59 {
                curr_hours = add_hour_safe(curr_hours, minutes / 60);
                remaining_minutes = minutes % 60;
            } else {
                remaining_minutes = minutes;
            }

            if cl_minutes + remaining_minutes > 59 {
                cl_minutes = (cl_minutes + remaining_minutes) % 60;
                curr_hours = add_hour_safe(curr_hours, 1);
            } else {
                cl_minutes = cl_minutes + remaining_minutes;
            }
        }

        return Clock { hours: curr_hours, minutes: cl_minutes };
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        let new_hours = add_hour_safe(self.hours, hours);
        Clock { hours: new_hours, minutes: self.minutes }
    }
}
