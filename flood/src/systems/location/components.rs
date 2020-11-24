use amethyst::core::ecs::{Component, VecStorage};
use amethyst::core::math::{Vector2};

pub struct Location {
  pos: Vector2<f32>,
  grid_pos: Vector2<u32>
}

impl Location {
  pub fn new(x: u32, y: u32) -> Self {
    Location {
      pos: Vector2::new(x as f32, y as f32),
      grid_pos: Vector2::new(x, y)
    }
  }
}

impl Component for Location {
  type Storage = VecStorage<Self>;
}