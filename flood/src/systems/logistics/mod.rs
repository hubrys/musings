use amethyst::core::SystemBundle;
use amethyst::Error;
use amethyst::shred::{World, DispatcherBuilder};
use crate::systems::logistics::graph::{Graph, Empty};
use amethyst::core::ecs::WorldExt;
pub use components::LogisticsNode;

mod components;
mod graph;
mod node_lifecycle;

pub type LogisticsGraph = Graph<Empty, f32>;

pub struct LogisticsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for LogisticsBundle {
  fn build(
    self,
    world: &mut World,
    builder: &mut DispatcherBuilder<'a, 'b>,
  ) -> Result<(), Error> {
    builder.add(
      node_lifecycle::NodeLifecycleSystem::new(),
      "node_lifecycle",
      &[]
    );
    Ok(())
  }
}