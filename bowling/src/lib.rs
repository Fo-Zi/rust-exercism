#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy)]
pub enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
}

pub struct BowlingGame {
    completed_frames: Vec<Frame>,
    current_frame: Option<u16>, // First throw of the current frame, if any
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            completed_frames: Vec::new(),
            current_frame: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.completed_frames.len() == 10 {
            return Err(Error::GameComplete);
        }
        match pins {
            pins if pins > 10 => return Err(Error::NotEnoughPinsLeft),
            _ => match self.current_frame {
                Some(first_throw) => match (first_throw + pins) {
                    sum_pins if sum_pins > 10 => return Err(Error::NotEnoughPinsLeft),
                    10 => {
                        self.completed_frames.push(Frame::Spare(first_throw, pins));
                        self.current_frame = None
                    }
                    _ => {
                        self.completed_frames.push(Frame::Open(first_throw, pins));
                        self.current_frame = None
                    }
                },
                None => {
                    if pins == 10 {
                        self.completed_frames.push(Frame::Strike);
                    } else {
                        self.current_frame = Some(pins)
                    }
                }
            },
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.completed_frames.is_empty() {
            return None;
        }

        let mut score = 0;
        let mut active_strikes: Vec<Option<u32>> = Vec::new();
        let mut last_frame_was_spare = false;
        for frame in self.completed_frames.iter() {
            match frame {
                Frame::Strike => score += 10,
                Frame::Open(first_throw, second_throw) => {
                    if !active_strikes.is_empty() {
                    } else {
                        score += *first_throw + *second_throw
                    }
                }
                Frame::Spare(first_throw, second_throw) => {
                    if !active_strikes.is_empty() {
                    } else {
                        score += *first_throw + second_throw
                    }
                }
            }
        }

        Some(score)

    }

}
