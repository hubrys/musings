use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlockingConfig {
}

impl Default for FlockingConfig {
  fn default() -> Self {
    FlockingConfig {
    }
  }
}