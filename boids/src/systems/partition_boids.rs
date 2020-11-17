use amethyst::core::{SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{Read, System, SystemData, WriteStorage, ReadStorage, Join, Entities, BitSet};
use amethyst::shred::{Write, Resources};
use crate::space_partition::TiledSpace;
use crate::components::{Boid, Movement, SpacePointer};
use amethyst::shrev::ReaderId;
use amethyst::core::ecs::storage::ComponentEvent;

#[derive(SystemDesc)]
pub struct PartitionBoidsSystem {
  added: BitSet,
  reader_id: Option<ReaderId<ComponentEvent>>
}

impl Default for PartitionBoidsSystem {
  fn default() -> Self {
    PartitionBoidsSystem {
      added: BitSet::new(),
      reader_id: None
    }
  }
}

impl<'s> System<'s> for PartitionBoidsSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Movement>,
    WriteStorage<'s, SpacePointer>,
    Write<'s, TiledSpace<u32>>,
  );

  fn run(&mut self, (ents, movements, mut pointers, mut grid): Self::SystemData) {
    self.added.clear();
    let events = pointers.channel().read(self.reader_id.as_mut().unwrap());
    for event in events {
      match event {
        ComponentEvent::Inserted(id) => {
          self.added.add(*id);
        }
        _ => {}
      }
    }

    for (ent, movement, pointer, _) in (&ents, &movements, &mut pointers, &self.added).join() {
      pointer.ptr = grid.add_elem(movement.position, ent.id());
    }

    for (ent, movement, pointer, _) in (&ents, &movements, &mut pointers, !&self.added).join() {
      grid.move_elem(&mut pointer.ptr, movement.position);
    }
  }

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.reader_id = Some(
      WriteStorage::<SpacePointer>::fetch(&res).register_reader()
    )
  }
}