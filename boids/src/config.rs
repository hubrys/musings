use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlockConfig {
  pub arena_size: [f32; 2],
  pub boid_count: u32,
  pub boid_max_speed: f32,
  pub local_group_distance: f32,
  pub boundary_offset: f32,
  pub boundary_force: f32,
  pub separation_weight: f32,
  pub separation_distance: f32,
  pub cohesion_weight: f32,
  pub alignment_weight: f32,
}

impl Default for FlockConfig {
  fn default() -> Self {
    FlockConfig {
      arena_size: [1200.0, 800.0],
      boid_count: 5,
      boid_max_speed: 100.0,
      local_group_distance: 500.0,
      boundary_offset: 100.0,
      boundary_force: 10.0,
      separation_weight: 1.0,
      separation_distance: 0.0,
      cohesion_weight: 1.0,
      alignment_weight: 1.0,
    }
  }
}