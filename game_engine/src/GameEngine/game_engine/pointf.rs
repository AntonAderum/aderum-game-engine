use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
pub struct Pointf {
    pub x: f64,
    pub y: f64,
}
impl Pointf {
    pub fn distance(&self, other: &Pointf) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }

    pub fn dot_product(&self, other: &Pointf) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x.powf(2.0) + self.y.powf(2.0))
    }
}

impl Add for Pointf {
    type Output = Pointf;

    fn add(self, other: Pointf) -> Pointf {
        Pointf {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pointf {
    type Output = Pointf;
    fn sub(self, other: Pointf) -> Pointf {
        Pointf {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl PartialEq for Pointf {
    fn eq<'a>(&'a self, other: &'a Pointf) -> bool {
        self.x == other.x && self.y == other.y
    }
}
