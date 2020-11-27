use amethyst::{
  core::ecs::{VecStorage, Component, FlaggedStorage, WriteStorage},
  assets::{PrefabData, ProgressCounter},
  derive::PrefabData,
  ecs::Entity,
  Error,
};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, PrefabData, Clone, Debug)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct LogisticsNode {
  reach: f32,
  travel_speed: f32,
}

impl LogisticsNode {
  pub fn new(reach: f32, travel_speed: f32) -> Self {
    LogisticsNode {
      reach,
      travel_speed
    }
  }

  pub fn reach(&self) -> f32 {
    self.reach
  }

  pub fn speed(&self) -> f32 {
    self.travel_speed
  }
}

impl Component for LogisticsNode {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
