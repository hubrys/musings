use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlockConfig {
  pub arena_size: [f32; 2],
  /// Number of boids to create
  pub boid_count: u32,
  /// Boid speed in units per second
  pub boid_speed: f32,
  /// Turn speed in degrees per second
  pub turning_speed: f32,
  pub separation_distance: f32,
  pub cohesion_distance: f32,
  pub alignment_distance: f32
}

impl Default for FlockConfig {
  fn default() -> Self {
    FlockConfig {
      arena_size: [1200.0, 800.0],
      boid_count: 5,
      boid_speed: 100.0,
      turning_speed: 180.0,
      separation_distance: 0.0,
      cohesion_distance: 0.0,
      alignment_distance: 0.0,
    }
  }
}