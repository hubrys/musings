use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, SystemData, WriteStorage, ReadStorage, Join};
use crate::systems::location::Location;

#[derive(SystemDesc)]
pub struct SyncTransformWithLocationSystem;

impl<'s> System<'s> for SyncTransformWithLocationSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Location>
  );

  fn run(&mut self, (mut transforms, locations): Self::SystemData) {
    (&mut transforms, &locations).join().for_each(|(transform, location)| {
      transform.set_translation_xyz(
        location.x(),
        location.y(),
        transform.translation().z);
    });
  }
}