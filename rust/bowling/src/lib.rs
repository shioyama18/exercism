type Score = u16;
type Pins = u16;

const MAX_PINS: Pins = 10;
const MAX_FRAMES: usize = 10;

/// Frame including the bonus rolls
struct Frame {
    begin: usize,
    end: usize,
}

struct Play {
    pins: Pins,
    is_second_throw: bool,
}

impl Play {
    fn new() -> Self {
        Play { 
            pins: MAX_PINS, 
            is_second_throw: false, 
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    play: Play,
    score: Vec<Score>,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            play: Play::new(),
            score: Vec::<Score>::new(),
            frames: Vec::<Frame>::new(),
        }
    }

    fn is_done(&self) -> bool {
        self.frames
            .get(MAX_FRAMES - 1)
            .map_or(false, |f| f.end == self.score.len())
    }            

    fn frame_done(&mut self, begin: usize, length: usize) {
        self.play = Play::new();
        self.frames.push(Frame { begin: begin, end: begin + length });
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_done() {
            return Err(Error::GameComplete);
        } else {
            if pins > self.play.pins {
                return Err(Error::NotEnoughPinsLeft);
            }
            let rolls = self.score.len();
            self.play.pins -= pins;

            match self.play {
                // Spare
                Play { pins: 0, is_second_throw: true } => self.frame_done(rolls - 1, 3),
                // Open
                Play { pins: _, is_second_throw: true } => self.frame_done(rolls - 1, 2),
                // Strike
                Play { pins: 0, is_second_throw: false } => self.frame_done(rolls, 3),
                _ => self.play.is_second_throw = true,
            }

            self.score.push(pins);
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_done() {
            Some(self.frames
               .iter()
               .take(MAX_FRAMES)
               .flat_map(|f| self.score[f.begin .. f.end].iter())
               .sum())
        } else {
            None
        }
    }
}
