use amethyst::ecs::{Component, VecStorage};
use amethyst::core::math::Vector2;
use amethyst::core::ecs::{NullStorage, FlaggedStorage};
use crate::space_partition::TiledSpacePointer;

#[derive(Default)]
pub struct Boid;

impl Component for Boid {
  type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Enemy;

impl Component for Enemy {
  type Storage = NullStorage<Self>;
}

pub struct Forces {
  pub force: Vector2<f32>,
}

impl Forces {
  pub fn force(&self) -> &Vector2<f32> {
    &self.force
  }
}

impl Default for Forces {
  fn default() -> Self {
    Forces {
      force: Vector2::new(20.0, 0.0)
    }
  }
}

impl Component for Forces {
  type Storage = VecStorage<Self>;
}

pub struct Movement {
  pub position: Vector2<f32>,
  pub velocity: Vector2<f32>,
}

impl Default for Movement {
  fn default() -> Self {
    Movement {
      position: Vector2::new(0.0, 0.0),
      velocity: Vector2::new(1.0, 0.0)
    }
  }
}

impl Component for Movement {
  type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct SpacePointer {
  pub ptr: TiledSpacePointer
}

impl Component for SpacePointer {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}