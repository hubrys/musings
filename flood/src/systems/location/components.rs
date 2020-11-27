use amethyst::core::ecs::{Component, VecStorage};
use amethyst::core::math::{Point2};

pub struct Location {
  pub pos: Point2<f32>
}

impl Component for Location {
  type Storage = VecStorage<Self>;
}

impl Location {
  pub fn new(x: f32, y: f32) -> Self {
    Location {
      pos: Point2::new(x, y)
    }
  }

  pub fn x(&self) -> f32 {
    self.pos.x
  }

  pub fn y(&self) -> f32 {
    self.pos.y
  }
}

