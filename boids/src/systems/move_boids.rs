use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::core::{Transform, Time, SystemDesc};
use amethyst::derive::SystemDesc;

use crate::components::{BoidIntent, TurnDirection, Boid};
use crate::utils::deg_to_rad;
use crate::config::FlockConfig;
use std::f32::consts::PI;

/// Moves boids based on their current state
#[derive(SystemDesc)]
pub struct MoveBoidsSystem;

impl<'s> System<'s> for MoveBoidsSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Boid>,
    ReadStorage<'s, BoidIntent>,
    Read<'s, Time>,
    Read<'s, FlockConfig>,
  );

  fn run(&mut self, (
    mut transforms,
    mut boids,
    mut boid_intents,
    time,
    flock
  ): Self::SystemData) {
    for (transform, boid, intent) in (&mut transforms, &mut boids, &boid_intents).join() {
      // apply the new rotation
      // move the boid the new direction based on speed
      let rotation_delta = intent.turning * flock.turning_speed * time.delta_seconds();
      boid.rotation += rotation_delta;
      if boid.rotation < -180.0 {
        boid.rotation = 180.0 - boid.rotation - 180.0;
      } else if boid.rotation > 180.0 {
        boid.rotation = -180.0 + boid.rotation - 180.0;
      }

      boid.rotation = boid.rotation.min(180.0).max(-180.0);
      boid.position.x += boid.rotation.to_radians().cos() * flock.boid_speed * time.delta_seconds();
      boid.position.y += boid.rotation.to_radians().sin() * flock.boid_speed * time.delta_seconds();

      if boid.position.x < 0.0 {
        boid.position.x = flock.arena_size[0];
      } else if boid.position.x > flock.arena_size[0] {
        boid.position.x = 0.0;
      }

      if boid.position.y < 0.0 {
        boid.position.y = flock.arena_size[1];
      } else if boid.position.y > flock.arena_size[1] {
        boid.position.y = 0.0;
      }

      transform.set_translation_xyz(boid.position.x, boid.position.y, 0.0);
      transform.set_rotation_2d(boid.rotation.to_radians() - PI / 2.0);
    }
  }
}