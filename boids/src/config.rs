use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlockConfig {
  pub arena_size: [f32; 2],
  pub boid_count: u32,
  pub boid_max_speed: f32,

  pub separation_weight: f32,
  pub separation_distance: f32,

  pub cohesion_weight: f32,
  pub cohesion_distance: f32,

  pub alignment_weight: f32,
  pub alignment_distance: f32,
}

impl Default for FlockConfig {
  fn default() -> Self {
    FlockConfig {
      arena_size: [1200.0, 800.0],
      boid_count: 5,
      boid_max_speed: 100.0,
      separation_weight: 1.0,
      separation_distance: 0.0,
      cohesion_weight: 1.0,
      cohesion_distance: 0.0,
      alignment_weight: 1.0,
      alignment_distance: 0.0,
    }
  }
}