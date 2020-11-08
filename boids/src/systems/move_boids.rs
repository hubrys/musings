use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::core::{Transform, Time, SystemDesc};
use amethyst::derive::SystemDesc;

use crate::components::{Boid, TurnDirection};
use crate::utils::deg_to_rad;
use crate::config::FlockConfig;

/// Moves boids based on their current state
#[derive(SystemDesc)]
pub struct MoveBoidsSystem;

impl<'s> System<'s> for MoveBoidsSystem {
  type SystemData = (
    ReadStorage<'s, Boid>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
    Read<'s, FlockConfig>,
  );

  fn run(&mut self, (
    boids,
    mut transforms,
    time,
    flock
  ): Self::SystemData) {
    for (boid, transform) in (&boids, &mut transforms).join() {
      // apply the new rotation
      // move the boid the new direction based on speed
      let rotation_delta = match boid.turn_direction {
        TurnDirection::None => 0.0,
        TurnDirection::Left => -flock.turning_speed,
        TurnDirection::Right => flock.turning_speed
      } * time.delta_seconds();

      transform.rotate_2d(deg_to_rad(rotation_delta));
      // Using move up, as we are pointing down the z-axis
      transform.move_up(flock.boid_speed * time.delta_seconds());

      let translation = transform.translation_mut();
      if translation.x < 0.0 {
        translation.x = flock.arena_size[0];
      } else if translation.x > flock.arena_size[0] {
        translation.x = 0.0;
      }

      if translation.y < 0.0 {
        translation.y = flock.arena_size[1];
      } else if translation.y > flock.arena_size[1] {
        translation.y = 0.0;
      }
    }
  }
}