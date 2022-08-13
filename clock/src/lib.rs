use std::{fmt};

pub struct Clock {
    hours: i32,
    minutes: i32
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

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut start_hours = match hours > 0 {
            true => {
                let mut start = 0;
                let mut counter = hours;
                while counter > 0 {
                    start = start + 1 % 23;
                    counter = counter - 1;
                }

                start
            },
            false => {
                let mut start = 23;
                let mut counter = hours;
                while counter > 0 {
                    start = start - 1 % 23;
                    counter = counter - 1;
                }

                start
            }
        };

        let start_minutes = match minutes > 0 {
            true => {
                if minutes > 59 {
                    let mut hours_to_add = minutes / 60;
                    let minutes_to_add  = minutes % 60;
                    
                    while hours_to_add > 0 {
                        start_hours = start_hours + 1 % 23;
                        hours_to_add = hours_to_add - 1;
                    }

                    minutes_to_add
                } else {
                    minutes
                }
            },
            false => {
                if minutes.abs() < 59 {
                    let remaining_minutes = minutes.abs() % 60;

                    60 - remaining_minutes                    
                } else {
                    let mut hours_to_remove = minutes.abs() / 60;
                    
                    while hours_to_remove > 0 {
                        start_hours = start_hours - 1 % 23;
                        hours_to_remove = hours_to_remove - 1;
                    }

                    let remaining_minutes = minutes.abs() % 60;

                    60 - remaining_minutes
                }
            }
        };


        Clock { hours: start_hours, minutes: start_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
