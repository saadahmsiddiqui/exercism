#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    InvalidNumberOfPins,
}
struct FrameHistory {
    first_turn_pins_knocked: i8,
    second_turn_pins_knocked: i8,
    third_turn_pins_knocked: i8,
}

fn new_empty_frame_history() -> FrameHistory {
    return FrameHistory {
        first_turn_pins_knocked: -1,
        second_turn_pins_knocked: -1,
        third_turn_pins_knocked: -1,
    };
}
pub struct BowlingGame {
    frame: u8,
    frame_history: Vec<FrameHistory>,
}

const GAME_FRAMES: usize = 9;

fn is_spare(frame: &FrameHistory) -> bool {
    return frame.first_turn_pins_knocked + frame.second_turn_pins_knocked == 10;
}

fn is_strike(frame: &FrameHistory) -> bool {
    return frame.first_turn_pins_knocked == 10;
}

fn calculate_frame_score(frame: &FrameHistory) -> u8 {
    frame.first_turn_pins_knocked as u8 + frame.second_turn_pins_knocked as u8
}

fn calculate_final_frame_score(frame: &FrameHistory) -> u8 {
    let mut score = frame.first_turn_pins_knocked as u8 + frame.second_turn_pins_knocked as u8;
    if frame.third_turn_pins_knocked != -1 {
        score = score + frame.third_turn_pins_knocked as u8;
    }
    score
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
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frame as usize > GAME_FRAMES {
            return Err(Error::GameComplete);
        }

        let frame_exists = match self.frame_history.get(self.frame as usize) {
            Some(_frame) => true,
            None => false,
        };

        if frame_exists && &self.frame_history[self.frame as usize].first_turn_pins_knocked + pins as i8 > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if frame_exists == true {
            if self.frame as usize == GAME_FRAMES {
                if self.frame_history[self.frame as usize].second_turn_pins_knocked == -1 {
                    self.frame_history[self.frame as usize].second_turn_pins_knocked = pins as i8;
                }
                if is_strike(&self.frame_history[self.frame as usize]) || is_spare(&self.frame_history[self.frame as usize]) {
                    if self.frame_history[self.frame as usize].third_turn_pins_knocked == -1 {
                        self.frame_history[self.frame as usize].third_turn_pins_knocked = pins as i8;
                        self.frame = self.frame + 1;
                    }
                } else {
                    self.frame = self.frame + 1;
                }
            } else {
                if self.frame_history[self.frame as usize].second_turn_pins_knocked == -1 {
                    self.frame_history[self.frame as usize].second_turn_pins_knocked = pins as i8;
                    self.frame = self.frame + 1;
                }
            }
        } else {
            let mut new_frame = new_empty_frame_history();
            new_frame.first_turn_pins_knocked = pins as i8;
            self.frame_history.push(new_frame);
            if pins == 10 {
                self.frame = self.frame + 1;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut frame = 0;
        let mut score: u16 = 0;

        while frame <= GAME_FRAMES {
            if frame == GAME_FRAMES {
                score = calculate_final_frame_score(&self.frame_history[frame]) as u16;
            } else {
                if is_strike(&self.frame_history[frame]) {
                    score = score + 10;

                    if frame + 2 <= GAME_FRAMES {
                        score = score + calculate_frame_score(&self.frame_history[frame + 1]) as u16;
                        score = score + calculate_frame_score(&self.frame_history[frame + 2]) as u16;
                    } else {
                        score = score + calculate_frame_score(&self.frame_history[frame + 1]) as u16;
                    }
                } else if is_spare(&self.frame_history[frame]) {
                    score = score + 10;
                    let next_frame = frame + 1;
                    if next_frame <= GAME_FRAMES {
                        score = score + self.frame_history[frame].first_turn_pins_knocked as u16;
                    }
                } else {
                    score = calculate_frame_score(&self.frame_history[frame]) as u16;
                }
            }

            frame = frame + 1;
        }

        Some(score)
    }
}
