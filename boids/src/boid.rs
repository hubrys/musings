use crate::math::{self, Vector2};

pub struct Boid {
  pub pos: Vector2
}

impl Boid {
  pub fn new() -> Boid {
    Boid {
      pos: [0.0, 0.0]
    }
  }
}