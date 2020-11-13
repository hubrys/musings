pub use move_boids::MoveBoidsSystem;
pub use direct_boids::DirectBoidsSystems;
use crate::components::Movement;
use amethyst::core::math::Vector2;

mod direct_boids;
mod move_boids;
pub mod boid_rules;

pub trait BoidRule {
  fn process_boid(&mut self, boid: &Movement, other_boid: &Movement, separation: f32);
  fn applied_force(&self) -> Vector2<f32>;
}