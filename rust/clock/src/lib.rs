use std::{cmp, fmt};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn add_hour_safe(current_hours: i32, hours: i32) -> i32 {
    match hours > 0 {
        true => (current_hours + hours) % 24,
        false => {
            let carried = hours.abs() % 24;
            (current_hours - carried + 24) % 24
        }
    }
}

fn calculate_minute_udpate(clock: &Clock, minutes: i32) -> (i32, i32) {
    let mut current_clock_hours = clock.hours;
    let mut current_clock_minutes = clock.minutes;

    let remaining_minutes = minutes.abs() % 60;

    let hours_to_add = minutes / 60;
    current_clock_hours = add_hour_safe(current_clock_hours, hours_to_add);

    match minutes > 0 {
        true => {
            if current_clock_minutes + remaining_minutes > 59 {
                current_clock_minutes = (current_clock_minutes + remaining_minutes) % 60;
                current_clock_hours = add_hour_safe(current_clock_hours, 1);
            } else {
                current_clock_minutes = current_clock_minutes + remaining_minutes;
            }
        }
        false => {
            if current_clock_minutes - remaining_minutes < 0 {
                current_clock_hours = add_hour_safe(current_clock_hours, -1);
                let minute_difference = (current_clock_minutes - remaining_minutes).abs();
                current_clock_minutes = 60 - minute_difference;
            } else {
                current_clock_minutes = current_clock_minutes - remaining_minutes;
            }
        }
    }

    (current_clock_hours, current_clock_minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock {
            hours: 0,
            minutes: 0,
        };
        clock = clock.add_hours(hours);
        clock = clock.add_minutes(minutes);
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = calculate_minute_udpate(&self, minutes);
        return Clock { hours, minutes };
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        let new_hours = add_hour_safe(self.hours, hours);
        Clock {
            hours: new_hours,
            minutes: self.minutes,
        }
    }
}
