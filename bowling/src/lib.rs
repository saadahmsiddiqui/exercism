#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    InvalidNumberOfPins,
}

#[derive(Clone)]
struct FrameHistory {
    frame_throws: [u16; 2],
    throw_index: usize,
}

struct FinalFrame {
    frame_throws: [u16; 3],
    throw_index: usize,
    third_throw_allowed: bool,
}
pub struct BowlingGame {
    current_frame: usize,
    final_frame: FinalFrame,
    frame_history: Vec<FrameHistory>,
}

const GAME_FRAMES: usize = 9;
const GAME_PINS: u16 = 10;

fn is_spare(frame: &FrameHistory) -> bool {
    frame.frame_throws[0] + frame.frame_throws[1] == GAME_PINS
}

fn is_strike(frame: &FrameHistory) -> bool {
    frame.frame_throws[0] == GAME_PINS
}

fn calculate_frame_score(frame: &FrameHistory) -> u16 {
    frame.frame_throws[0] + frame.frame_throws[1]
}

fn calculate_final_frame_score(frame: &FinalFrame) -> u16 {
    frame.frame_throws[0] + frame.frame_throws[1] + frame.frame_throws[2]
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut frame_history = Vec::<FrameHistory>::with_capacity(10);

        for _ in 0..GAME_FRAMES {
            frame_history.push(FrameHistory {
                frame_throws: [0, 0],
                throw_index: 0,
            });
        }

        BowlingGame {
            current_frame: 0,
            final_frame: FinalFrame {
                frame_throws: [0, 0, 0],
                third_throw_allowed: false,
                throw_index: 0,
            },
            frame_history,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.current_frame > GAME_FRAMES {
            return Err(Error::GameComplete);
        }

        if self.current_frame == GAME_FRAMES {
            let throw_index = self.final_frame.throw_index;
            match self.final_frame.throw_index {
                0 => {
                    self.final_frame.frame_throws[throw_index as usize] = pins;
                    self.final_frame.throw_index = self.final_frame.throw_index + 1;
                }
                1 => {
                    if self.final_frame.frame_throws[1] + self.final_frame.frame_throws[0] > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }

                    self.final_frame.frame_throws[throw_index] = pins;
                    self.final_frame.throw_index = self.final_frame.throw_index + 1;

                    if self.final_frame.frame_throws[0] == GAME_PINS
                        || self.final_frame.frame_throws[1] == GAME_PINS
                        || self.final_frame.frame_throws[0] + self.final_frame.frame_throws[1]
                            == GAME_PINS
                    {
                        self.final_frame.third_throw_allowed = true;
                    } else {
                        self.current_frame = self.current_frame + 1;
                    }
                }
                2 => {
                    if self.final_frame.third_throw_allowed {
                        self.final_frame.frame_throws[throw_index] = pins;
                    }
                    self.final_frame.throw_index = self.final_frame.throw_index + 1;
                    self.current_frame = self.current_frame + 1;
                }
                _ => {
                    return Err(Error::GameComplete);
                }
            }
        } else if self.current_frame < GAME_FRAMES {
            let frame = self.current_frame;
            let throw_index = self.frame_history[frame].throw_index;

            if throw_index == 0 {
                self.frame_history[frame].frame_throws[throw_index] = pins;
                self.frame_history[frame].throw_index = self.frame_history[frame].throw_index + 1;
                if pins == GAME_PINS {
                    self.current_frame = self.current_frame + 1;
                }
            }
            if throw_index == 1 {
                if self.frame_history[frame].frame_throws[0]
                    + self.frame_history[frame].frame_throws[1]
                    > 10
                {
                    return Err(Error::NotEnoughPinsLeft);
                }

                self.frame_history[frame].frame_throws[throw_index] = pins;
                self.current_frame = self.current_frame + 1;
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_history[0].throw_index == 0 {
            return None;
        }

        let mut score: u16 = 0;
        for i in 0..GAME_FRAMES {
            if is_spare(&self.frame_history[i]) {
                let next_frame = i + 1;
                let next_frame_is_final_frame = next_frame == GAME_FRAMES;

                if next_frame_is_final_frame {
                    let current_score = GAME_PINS as u16 + self.final_frame.frame_throws[0];
                    score = score + current_score;
                } else {
                    let current_score =
                        GAME_PINS as u16 + self.frame_history[next_frame].frame_throws[0];
                    score = score + current_score;
                }
            } else if is_strike(&self.frame_history[i]) {
                let next_two_frames = i + 2;
                let this_frame_score = GAME_PINS as u16;

                if next_two_frames == GAME_FRAMES {
                    score = score
                        + this_frame_score
                        + calculate_frame_score(&self.frame_history[next_two_frames - 1])
                        + self.final_frame.frame_throws[0];
                } else if next_two_frames > GAME_FRAMES {
                    score = score
                        + this_frame_score
                        + self.final_frame.frame_throws[0]
                        + self.final_frame.frame_throws[1];
                } else {
                    score = score
                        + this_frame_score
                        + calculate_frame_score(&self.frame_history[i + 1])
                        + calculate_frame_score(&self.frame_history[i + 2]);
                }
            } else {
                score = score + calculate_frame_score(&self.frame_history[i]);
            }
        }

        Some(score + calculate_final_frame_score(&self.final_frame))
    }
}
