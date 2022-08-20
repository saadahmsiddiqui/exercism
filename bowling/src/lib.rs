#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
struct FrameHistory {
    first_turn_pins_knocked: u8,
    second_turn_pins_knoced: u8,
    is_spare: bool,
    is_strike: bool,
}
pub struct BowlingGame {
    frame: u8,
    frame_hitory: Vec<FrameHistory>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frame: 0, frame_hitory: Vec::<FrameHistory>::with_capacity(10) }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        unimplemented!("Record that {} pins have been scored", pins);
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
