use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::core::{Transform, Time, SystemDesc};
use amethyst::derive::SystemDesc;

use crate::components::{Boid, TurnDirection};
use crate::utils::deg_to_rad;

/// In units per second
const BOID_SPEED: f32 = 10.0;
/// In degrees per second
const TURN_SPEED: f32 = 180.0;

/// Moves boids based on their current state
#[derive(SystemDesc)]
pub struct MoveBoidsSystem;

impl<'s> System<'s> for MoveBoidsSystem {
  type SystemData = (
    Read<'s, Time>,
    ReadStorage<'s, Boid>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (time, boids, mut transforms): Self::SystemData) {
    for (boid, transform) in (&boids, &mut transforms).join() {
      // apply the new rotation
      // move the boid the new direction based on speed
      let rotation_delta = match boid.turn_direction {
        TurnDirection::None => 0.0,
        TurnDirection::Left => -TURN_SPEED,
        TurnDirection::Right => TURN_SPEED
      } * time.delta_seconds();

      transform.rotate_2d(deg_to_rad(rotation_delta));
      // Using move up, as we are pointing down the z-axis
      transform.move_up(BOID_SPEED * time.delta_seconds());
    }
  }
}