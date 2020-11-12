pub use move_boids::MoveBoidsSystem;
pub use direct_boids::DirectBoidsSystems;
use crate::components::Boid;
use amethyst::core::math::Vector2;

mod direct_boids;
mod move_boids;
pub mod boid_rules;

pub trait BoidRule {
  fn process_boid(&mut self, boid: &Boid, other_boid: &Boid, separation: f32);
  fn applied_force(&self) -> Vector2<f32>;
}