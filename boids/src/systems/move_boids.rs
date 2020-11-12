use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;

use crate::components::{BoidIntent, Boid};
use crate::config::FlockConfig;
use std::f32::consts::PI;
use amethyst::core::num::Pow;

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
    boid_intents,
    time,
    flock
  ): Self::SystemData) {
    for (transform, boid, intent) in (&mut transforms, &mut boids, &boid_intents).join() {
      let force = intent.force();

      boid.velocity.x += force.x * time.delta_seconds();
      boid.velocity.y += force.y * time.delta_seconds();

      // Clamping velocity
      if boid.velocity.magnitude_squared() > flock.boid_max_speed.pow(2) {
        boid.velocity = boid.velocity.normalize() * flock.boid_max_speed;
      }

      boid.position.x += boid.velocity.x * time.delta_seconds();
      boid.position.y += boid.velocity.y * time.delta_seconds();

      // if boid.position.x < 0.0 {
      //   boid.position.x = flock.arena_size[0];
      // } else if boid.position.x > flock.arena_size[0] {
      //   boid.position.x = 0.0;
      // }
      //
      // if boid.position.y < 0.0 {
      //   boid.position.y = flock.arena_size[1];
      // } else if boid.position.y > flock.arena_size[1] {
      //   boid.position.y = 0.0;
      // }
      let angle = boid.velocity.y.atan2(boid.velocity.x);
      transform.set_translation_xyz(boid.position.x, boid.position.y, 0.0);
      transform.set_rotation_2d(angle - PI / 2.0);
    }
  }
}