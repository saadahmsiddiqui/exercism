pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut start_hours = match hours > 0 {
            true => {
                let mut start = 0;
                let mut counter = hours;
                while (counter > 0) {
                    start = start + 1 % 23;
                    counter = counter - 1;
                }

                start
            },
            false => {
                let mut start = 0;
                let mut counter = hours;
                while (counter > 0) {
                    start = start + 1 % 23;
                    counter = counter - 1;
                }

                start
            }
        };

        let start_minutes = match minutes > 0 {
            true => {
                0
            },
            false => {
                0
            }
        };


        Clock { hours: start_hours, minutes: start_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
