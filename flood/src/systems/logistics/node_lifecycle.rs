use amethyst::core::{SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, SystemData, BitSet, WriteStorage, ReadStorage, Join, Entities};
use amethyst::core::ecs::storage::ComponentEvent;
use amethyst::shrev::ReaderId;
use amethyst::shred::{World, Write};
use crate::systems::logistics::components::LogisticsNode;
use crate::systems::logistics::LogisticsGraph;
use crate::systems::logistics::graph::Empty;
use crate::systems::location::Location;
use amethyst::core::math::{distance, Point3};
use amethyst::renderer::debug_drawing::DebugLinesComponent;
use amethyst::renderer::palette::Srgba;

#[derive(SystemDesc, Default)]
pub struct NodeLifecycleSystem {
  added: BitSet,
  removed: BitSet,
  reader_id: Option<ReaderId<ComponentEvent>>
}

impl NodeLifecycleSystem {
  pub fn new() -> Self {
    Self::default()
  }
}

impl<'s> System<'s> for NodeLifecycleSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, LogisticsNode>,
    ReadStorage<'s, Location>,
    WriteStorage<'s, DebugLinesComponent>,
    Write<'s, LogisticsGraph>,
  );

  fn run(&mut self, (
    entities,
    nodes,
    locations,
    mut debug_lines_storage,
    mut graph
  ): Self::SystemData) {
    self.added.clear();
    self.removed.clear();

    let events = nodes.channel().read(self.reader_id.as_mut().unwrap());
    for event in events {
      match event {
        ComponentEvent::Inserted(id) => { self.added.add(*id); }
        ComponentEvent::Removed(id) => { self.removed.add(*id); }
        _ => {}
      }
    }

    // Adding Nodes
    for (node, location, id) in (&nodes, &locations, &self.added).join() {
      graph.add_node(id, Empty);
      for (other_node, other_location, other_ent, _) in (
        &nodes,
        &locations,
        &entities,
        !&self.added
      ).join() {
        if id == other_ent.id() {
          continue;
        }

        let distance = distance(&location.pos, &other_location.pos);
        let max_allowed_distance = node.reach().min(other_node.reach());
        println!("NEW NODE {} {}", distance, max_allowed_distance);

        // We can connect this bad boy
        if distance < max_allowed_distance {
          let slower_speed = node.speed().min(other_node.speed());
          let weight = distance / slower_speed;
          // Stored as bidirectional
          graph.add_edge(id, other_ent.id(), weight);
          graph.add_edge(other_ent.id(), id, weight);

          let mut debug_lines = DebugLinesComponent::new();
          debug_lines.add_line(
            Point3::new(location.pos.x, location.pos.y, 0.0),
            Point3::new(other_location.pos.x, other_location.pos.y, 0.0),
            Srgba::new(0.0, 0.0, 1.0, 1.0)
          );
          entities
            .build_entity()
            .with(debug_lines, &mut debug_lines_storage)
            .build();
        }
      }
    }

    // // Removing Nodes
    // for (node, location, id) in (&nodes, &locations, &self.removed).join() {
    //   graph.remove_node(id, Empty);
    // }
  }

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    self.reader_id = Some(
      WriteStorage::<LogisticsNode>::fetch(&world).register_reader()
    );
  }
}