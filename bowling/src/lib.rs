#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    InvalidNumberOfPins,
}
struct FrameHistory {
    first_turn_pins_knocked: i8,
    second_turn_pins_knocked: i8,
    is_spare: bool,
    is_strike: bool,
}

fn new_empty_frame_history() -> FrameHistory {
    return FrameHistory {
        first_turn_pins_knocked: -1,
        second_turn_pins_knocked: -1,
        is_strike: false,
        is_spare: false,
    };
}
pub struct BowlingGame {
    frame: u8,
    frame_history: Vec<FrameHistory>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame: 0,
            frame_history: Vec::<FrameHistory>::with_capacity(10),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::InvalidNumberOfPins);
        }

        if self.frame + 1 >= 10 {
            return Err(Error::GameComplete);
        }

        if self.frame == 0 {
            if self.frame_history.len() == 0 {
                let is_strike = pins == 10;
                let mut first_frame = new_empty_frame_history();
                first_frame.is_strike = is_strike;
                first_frame.first_turn_pins_knocked = pins as i8;
                self.frame_history.push(first_frame);
                if is_strike {
                    self.frame = self.frame + 1;
                }
            } else {
                self.frame_history[self.frame as usize].is_spare = false;
                if self.frame_history[self.frame as usize].first_turn_pins_knocked + pins as i8
                    == 10
                {
                    self.frame_history[self.frame as usize].is_spare = true;
                }
                self.frame_history[self.frame as usize].second_turn_pins_knocked = pins as i8;
                self.frame = self.frame + 1;
            }
        } else {
            let last_frame = (self.frame - 1) as usize;
            let is_complete_last_frame =
                self.frame_history[last_frame].second_turn_pins_knocked != -1;
            if is_complete_last_frame {
                let mut new_current_frame = new_empty_frame_history();
                new_current_frame.is_strike = pins == 10;
                new_current_frame.is_spare = false;
                new_current_frame.first_turn_pins_knocked = pins as i8;
                self.frame_history[self.frame as usize] = new_current_frame;
            } else {
                self.frame_history[self.frame as usize].is_spare = false;
                if self.frame_history[self.frame as usize].first_turn_pins_knocked + pins as i8
                    == 10
                {
                    self.frame_history[self.frame as usize].is_spare = true;
                }
                self.frame_history[self.frame as usize].second_turn_pins_knocked = pins as i8;
                self.frame = self.frame + 1;
            }
        }
        unimplemented!("Return the score if the game is complete, or None if not.");
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
