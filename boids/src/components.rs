use amethyst::ecs::{NullStorage, Component, VecStorage};
use amethyst::core::math::Vector2;

pub enum TurnDirection {
  None,
  Left,
  Right,
}

pub struct Boid {
  pub turn_direction: TurnDirection,
}

impl Default for Boid {
  fn default() -> Self {
    Boid {
      turn_direction: TurnDirection::None
    }
  }
}

impl Component for Boid {
  type Storage = VecStorage<Self>;
}