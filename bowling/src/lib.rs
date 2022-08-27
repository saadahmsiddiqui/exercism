#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    InvalidNumberOfPins,
}
struct FrameHistory {
    first_turn_pins_knocked: u16,
    second_turn_pins_knocked: u16,
    third_turn_pins_knocked: u16,
}

fn new_empty_frame_history() -> FrameHistory {
    return FrameHistory {
        first_turn_pins_knocked: 0,
        second_turn_pins_knocked: 0,
        third_turn_pins_knocked: 0,
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

fn calculate_frame_score(frame: &FrameHistory) -> u16 {
    frame.first_turn_pins_knocked + frame.second_turn_pins_knocked
}

fn calculate_final_frame_score(frame: &FrameHistory) -> u16 {
    let score = frame.first_turn_pins_knocked
        + frame.second_turn_pins_knocked
        + frame.third_turn_pins_knocked;
    score
}

fn print_frame(frame: &FrameHistory) {
    println!("{} {} {}",
    &frame.first_turn_pins_knocked,
    &frame.second_turn_pins_knocked,
    &frame.third_turn_pins_knocked);
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame: 0,
            frame_history: Vec::<FrameHistory>::with_capacity(10),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let current_frame: usize = self.frame as usize;

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if current_frame > GAME_FRAMES {
            return Err(Error::GameComplete);
        }

        let frame_exists = match self.frame_history.get(current_frame) {
            Some(_frame) => true,
            None => false,
        };

        if frame_exists && current_frame < GAME_FRAMES as usize && &self.frame_history[current_frame].first_turn_pins_knocked + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        
        if frame_exists == true {
            if current_frame == GAME_FRAMES {
                if self.frame_history[current_frame].second_turn_pins_knocked == 99 {
                    self.frame_history[current_frame].second_turn_pins_knocked = pins;
                    return Ok(())
                } else {
                    if self.frame_history[current_frame].first_turn_pins_knocked == 10
                        || self.frame_history[current_frame].first_turn_pins_knocked
                            + self.frame_history[current_frame].second_turn_pins_knocked
                            == 10
                    {
                        self.frame_history[current_frame].third_turn_pins_knocked = pins;
                    }
                    self.frame = self.frame + 1;
                }
            } else {
                self.frame_history[current_frame].second_turn_pins_knocked = pins;
                self.frame = self.frame + 1;
            }
        } else {
            let mut new_frame = new_empty_frame_history();
            new_frame.first_turn_pins_knocked = pins;
            self.frame_history.push(new_frame);
            if pins == 10 && self.frame < GAME_FRAMES as u8 {
                self.frame = self.frame + 1;
            }
            if self.frame == GAME_FRAMES as u8 {
                self.frame_history[self.frame as usize].second_turn_pins_knocked = 99;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_history.len() == 0 {
            return None;
        }

        let mut frame = 0;
        let mut score: u16 = 0;

        while frame <= GAME_FRAMES {
            if frame == GAME_FRAMES {
                score = score +calculate_final_frame_score(&self.frame_history[frame]) as u16;
            } else {
                if is_strike(&self.frame_history[frame]) {
                    score = score + 10;

                    if frame + 2 <= GAME_FRAMES {
                        score =
                            score + calculate_frame_score(&self.frame_history[frame + 1]) as u16;
                        score =
                            score + calculate_frame_score(&self.frame_history[frame + 2]) as u16;
                    } else {
                        score =
                            score + calculate_frame_score(&self.frame_history[frame + 1]) as u16;
                    }
                } else if is_spare(&self.frame_history[frame]) {
                    score = score + 10;
                    let next_frame = frame + 1;
                    if next_frame <= GAME_FRAMES {
                        score = score + self.frame_history[frame].first_turn_pins_knocked as u16;
                    }
                } else {
                    score = score + calculate_frame_score(&self.frame_history[frame]) as u16;
                }
            }

            frame = frame + 1;
        }

        Some(score)
    }
}
