#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Mark {
    Spare,
    Strike,
    Open,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Frame {
    first: Option<u16>,
    second: Option<u16>,
    mark: Option<Mark>,
}

impl Frame {
    fn new() -> Self {
        Self {
            first: None,
            second: None,
            mark: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.mark.is_some()
    }

    fn add_roll_and_mark(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            Err(Error::NotEnoughPinsLeft)
        } else {
            if self.first.is_none() {
                self.first = match pins {
                    10 => {
                        self.mark = Some(Mark::Strike);
                        Some(pins)
                    }
                    0..=9 => Some(pins),
                    _ => return Err(Error::NotEnoughPinsLeft),
                };
            } else if let Some(first_pins) = self.first {
                self.second = match pins + first_pins {
                    10 => {
                        self.mark = Some(Mark::Spare);
                        Some(pins)
                    }
                    0..=9 => {
                        self.mark = Some(Mark::Open);
                        Some(pins)
                    }
                    _ => return Err(Error::NotEnoughPinsLeft),
                }
            }
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    bonus_throws: (Option<u16>, Option<u16>),
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![Frame::new(); 1],
            bonus_throws: (None, None),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.game_is_complete() {
            return Err(Error::GameComplete);
        } else if self.frames.len() == 10 && self.frames.last().unwrap().is_complete() {
            // handle bonus throws
            match (
                self.frames.last().unwrap().mark.as_ref(),
                self.bonus_throws,
                pins,
            ) {
                (Some(Mark::Spare), _, 0..=10) => {
                    self.bonus_throws.0 = Some(pins);
                }
                (Some(Mark::Strike), (None, _), 0..=10) => {
                    self.bonus_throws.0 = Some(pins);
                }
                (Some(Mark::Strike), (Some(throw1_pins), None), pins)
                    if (throw1_pins == 10 && pins <= 10) || throw1_pins + pins <= 10 =>
                {
                    self.bonus_throws.1 = Some(pins);
                }
                _ => return Err(Error::NotEnoughPinsLeft),
            }
        } else {
            if self.frames.last().unwrap().is_complete() {
                self.frames.push(Frame::new());
            }

            self.frames.last_mut().unwrap().add_roll_and_mark(pins)?;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_is_complete() {
            return None;
        }

        Some(self.frames.iter().enumerate().fold(0, |acc, (i, frame)| {
            acc + match frame.mark.as_ref().unwrap() {
                Mark::Spare => {
                    if i == 9 {
                        10 + self.bonus_throws.0.unwrap()
                    } else {
                        10 + self.frames[i + 1].first.unwrap()
                    }
                }
                Mark::Strike => match i {
                    9 => 10 + self.bonus_throws.0.unwrap() + self.bonus_throws.1.unwrap(),
                    8 => {
                        10 + self.frames[i + 1].first.unwrap()
                            + match self.frames[i + 1].mark.as_ref().unwrap() {
                                Mark::Strike => self.bonus_throws.0.unwrap(),
                                _ => self.frames[i + 1].second.unwrap(),
                            }
                    }
                    _ => {
                        10 + self.frames[i + 1].first.unwrap()
                            + match self.frames[i + 1].mark.as_ref().unwrap() {
                                Mark::Strike => self.frames[i + 2].first.unwrap(),
                                _ => self.frames[i + 1].second.unwrap(),
                            }
                    }
                },
                Mark::Open => frame.first.unwrap() + frame.second.unwrap(),
            }
        }))
    }

    fn game_is_complete(&self) -> bool {
        self.frames.len() == 10
            && self.frames.last().unwrap().is_complete()
            && matches!(
                (self.frames.last().unwrap().mark.as_ref(), self.bonus_throws),
                (Some(Mark::Spare), (Some(_), None))
                    | (Some(Mark::Strike), (Some(_), Some(_)))
                    | (Some(Mark::Open), (None, None))
            )
    }
}
