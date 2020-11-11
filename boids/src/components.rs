use amethyst::ecs::{Component, VecStorage};
use amethyst::core::math::Vector2;

pub struct BoidIntent {
  pub force: Vector2<f32>,
}

impl BoidIntent {
  pub fn force(&self) -> &Vector2<f32> {
    &self.force
  }
}

impl Default for BoidIntent {
  fn default() -> Self {
    BoidIntent {
      force: Vector2::new(20.0, 0.0)
    }
  }
}

impl Component for BoidIntent {
  type Storage = VecStorage<Self>;
}

pub struct Boid {
  pub position: Vector2<f32>,
  pub velocity: Vector2<f32>,
}

impl Default for Boid {
  fn default() -> Self {
    Boid {
      position: Vector2::new(0.0, 0.0),
      velocity: Vector2::new(1.0, 0.0)
    }
  }
}

impl Component for Boid {
  type Storage = VecStorage<Self>;
}
