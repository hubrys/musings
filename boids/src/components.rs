use amethyst::ecs::{NullStorage, Component, VecStorage};
use amethyst::core::math::Vector2;

pub enum TurnDirection {
  None,
  Left,
  Right,
}

#[derive(Default)]
pub struct BoidIntent {
  pub turning: f32,
}

impl Component for BoidIntent {
  type Storage = VecStorage<Self>;
}

pub struct Boid {
  pub position: Vector2<f32>,
  pub rotation: f32
}

impl Default for Boid {
  fn default() -> Self {
    Boid {
      position: Vector2::new(0.0, 0.0),
      rotation: 0.0
    }
  }
}

impl Component for Boid {
  type Storage = VecStorage<Self>;
}
