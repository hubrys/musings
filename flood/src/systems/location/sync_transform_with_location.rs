use amethyst::core::{SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{Read, System, SystemData};

#[derive(SystemDesc)]
pub struct SyncTransformWithLocationSystem;

impl<'s> System<'s> for SyncTransformWithLocationSystem {
  type SystemData = ();

  fn run(&mut self, (): Self::SystemData) {}
}