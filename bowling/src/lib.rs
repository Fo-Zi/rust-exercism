use Error::*;
use Frame::*;
use State::*;

type Pins = u16;
const MAX_PINS: Pins = 10;
const MAX_FRAMES: usize = 10;

#[derive(Default)]
pub struct BowlingGame {
    state: State,            // current state of the game
    prev_frames: Vec<Frame>, // previous completed frames
}

// Possible states of the game
enum State {
    FirstBallOfFrame,
    SecondBallOfFrame(Pins),
    FirstBonusBall,
    SecondBonusBall(Pins),
    Complete(Option<(Pins, Option<Pins>)>),
}

impl Default for State {
    fn default() -> Self {
        FirstBallOfFrame
    }
}

// Possible frames
#[derive(PartialEq, Eq)]
enum Frame {
    Strike,
    Spare(Pins),
    Open(Pins, Pins),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a completed frame to the game and determines the next state.
    fn close_frame(&mut self, result: Frame) {
        self.state = if self.prev_frames.len() == MAX_FRAMES - 1 {
            // we are adding the last frame
            match result {
                Strike | Spare(_) => FirstBonusBall,
                Open(_, _) => Complete(None),
            }
        } else {
            FirstBallOfFrame
        };
        self.prev_frames.push(result);
    }

    /// Add the roll of a ball to the game, returns an Error if the number of
    /// pins is invalid or if the game is already complete.
    pub fn roll(&mut self, pins: Pins) -> Result<(), Error> {
        match self.state {
            FirstBallOfFrame => match pins {
                MAX_PINS => self.close_frame(Strike),
                0..=MAX_PINS => self.state = SecondBallOfFrame(pins),
                _ => return Err(NotEnoughPinsLeft),
            },
            SecondBallOfFrame(prev_pins) => match prev_pins + pins {
                MAX_PINS => self.close_frame(Spare(prev_pins)),
                0..=MAX_PINS => self.close_frame(Open(prev_pins, pins)),
                _ => return Err(NotEnoughPinsLeft),
            },
            FirstBonusBall => {
                assert!(self.prev_frames.len() == MAX_FRAMES);
                if pins > MAX_PINS {
                    return Err(NotEnoughPinsLeft);
                }

                // look at what the final frame was
                match self.prev_frames[MAX_FRAMES - 1] {
                    Strike => self.state = SecondBonusBall(pins),
                    Spare(_) => self.state = Complete(Some((pins, None))),
                    _ => unreachable!(),
                }
            }
            SecondBonusBall(prev_pins) => {
                assert!(self.prev_frames.len() == MAX_FRAMES);
                assert!(self.prev_frames[MAX_FRAMES - 1] == Strike);
                assert!(prev_pins <= MAX_PINS);

                if (prev_pins == MAX_PINS && pins <= MAX_PINS)
                    || (prev_pins < MAX_PINS && prev_pins + pins <= MAX_PINS)
                {
                    // it is a valid number of pins
                    self.state = Complete(Some((prev_pins, Some(pins))));
                } else {
                    return Err(NotEnoughPinsLeft);
                }
            }
            Complete(_) => return Err(GameComplete),
        }
        Ok(())
    }

    /// Returns the pins of the two balls thrown after the given frame.
    fn next_two_balls_after_frame(
        &self,
        index: usize,
    ) -> Option<(Pins, Option<Pins>)> {
        if index >= MAX_FRAMES {
            // invalid index
            None
        } else if let Some(frame) = self.prev_frames.get(index + 1) {
            // looking at the next frame
            match frame {
                Strike => Some((
                    MAX_PINS,
                    self.next_two_balls_after_frame(index + 1)
                        .map(|(ball2, _)| ball2),
                )),
                Spare(ball1) => Some((*ball1, Some(MAX_PINS - *ball1))),
                Open(ball1, ball2) => Some((*ball1, Some(*ball2))),
            }
        } else if let Complete(fill_balls) = self.state {
            // game is complete
            assert!(index == MAX_FRAMES - 1);
            fill_balls
        } else {
            None
        }
    }

    /// Returns the score of a frame if it can be calculated.
    fn score_of_frame(&self, index: usize) -> Option<u16> {
        if let Some(frame) = self.prev_frames.get(index) {
            match frame {
                Strike => self.next_two_balls_after_frame(index).and_then(
                    |(bonus1, option_bonus2)| {
                        option_bonus2.map(|bonus2| MAX_PINS + bonus1 + bonus2)
                    },
                ),
                Spare(_) => self.next_two_balls_after_frame(index)
                    .map(|(bonus, _)| MAX_PINS + bonus),
                Open(ball1, ball2) => Some(ball1 + ball2),
            }
        } else {
            None
        }
    }

    /// Returns the score of the game if it is completed.
    pub fn score(&self) -> Option<u16> {
        if let Some(scores) = (0..MAX_FRAMES)
            .map(|i| self.score_of_frame(i))
            .inspect(|score| println!("frame score: {:?}", score))
            .collect::<Option<Vec<_>>>()
        {
            Some(scores.iter().sum())
        } else {
            None
        }
    }
}