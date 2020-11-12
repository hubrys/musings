use amethyst::core::math::Vector2;
use crate::systems::BoidRule;
use crate::components::Boid;

pub struct SeparationRule {
  accumulator: Vector2<f32>
}

impl SeparationRule {
  fn new() -> Self {
    SeparationRule {
      accumulator: Vector2::zeros()
    }
  }
}

impl BoidRule for SeparationRule {
  fn process_boid(boid: &Boid, other_boid: &Boid, separation: f32) {

  }

  fn applied_force(&self) -> Vector2<f32> {
    unimplemented!()
  }
}