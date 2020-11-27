use amethyst::{
  core::ecs::{VecStorage, Component, FlaggedStorage, WriteStorage},
  assets::{PrefabData, ProgressCounter},
  derive::PrefabData,
  ecs::Entity,
  Error,
};
use serde::{Deserialize, Serialize};
use crate::systems::logistics::LogisticsNode;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::palette::Srgba;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct NodePrefabData {
  log_node: LogisticsNode,
  tint: TintPrefab
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TintPrefab {
  data: (f32, f32, f32, f32)
}

impl<'a> PrefabData<'a> for TintPrefab {
  type SystemData = WriteStorage<'a, Tint>;
  type Result = ();

  fn add_to_entity(
    &self,
    entity: Entity,
    system_data: &mut Self::SystemData,
    entities: &[Entity],
    children: &[Entity]
  ) -> Result<Self::Result, Error> {
    let tint = Tint(Srgba::from_components(self.data));
    system_data.insert(entity, tint);
    Ok(())
  }
}