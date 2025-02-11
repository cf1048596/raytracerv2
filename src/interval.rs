use crate::helper::{self, INFINITY};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new_empty() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max <= x
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        match x {
            n if n < self.min => self.min,
            n if n > self.max => self.max,
            _ => x,
        }
    }
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: helper::INFINITY,
    max: -helper::INFINITY,
};

pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: -helper::INFINITY,
    max: helper::INFINITY,
};
