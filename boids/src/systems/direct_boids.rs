use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::derive::SystemDesc;
use amethyst::core::math::{Vector2};
use amethyst::input::{InputHandler, StringBindings};
use crate::components::{BoidIntent, Boid};
use crate::config::FlockConfig;
use amethyst::core::ecs::Entities;

/// Moves boids based on simple rules
/// rules:
/// - Separation - avoid crowding neighbors
/// - Alignment - go in the same direction as neighbors
/// - Cohesion - move closer to center mass of local neighbors
#[derive(SystemDesc)]
pub struct DirectBoidsSystems;

impl<'s> System<'s> for DirectBoidsSystems {
  type SystemData = (
    Read<'s, InputHandler<StringBindings>>,
    Read<'s, FlockConfig>,
    Entities<'s>,
    ReadStorage<'s, Boid>,
    WriteStorage<'s, BoidIntent>,
  );

  fn run(&mut self, (
    _input,
    flock,
    ents,
    boids,
    mut boid_intents
  ): Self::SystemData) {
    for (ent, boid, boid_intent) in (&ents, &boids, &mut boid_intents).join() {
      // Center of mass
      let mut com = Vector2::zeros();
      let mut cohesion_count = 0;
      let mut separation_vec = Vector2::zeros();

      let mut velocity_count = 0;
      let mut average_velocity = Vector2::zeros();

      for (other_ent, other_boid) in (&ents, &boids).join() {
        // Don't include self in calculations
        if ent.id() == other_ent.id() {
          continue;
        }

        let boid_separation = distance(&boid.position, &other_boid.position);
        // COHESION
        if boid_separation < flock.cohesion_distance {
          cohesion_count += 1;
          com += other_boid.position;
        }

        // SEPARATION
        if boid_separation < flock.separation_distance {
          separation_vec = boid.position - other_boid.position;
        }

        // ALIGNMENT
        if boid_separation < flock.alignment_distance {
          velocity_count += 1;
          average_velocity += other_boid.velocity;
        }
      }
      if cohesion_count != 0 {
        com /= cohesion_count as f32;
      }
      if velocity_count != 0 {
        average_velocity /= velocity_count as f32;
      }
      let cohesion_vec = com - boid.position;
      boid_intent.force =
        cohesion_vec * flock.cohesion_weight +
          separation_vec * flock.separation_weight +
          average_velocity * flock.alignment_weight;
    }
  }
}

fn distance(lhs: &Vector2<f32>, rhs: &Vector2<f32>) -> f32 {
  (lhs - rhs).norm()
}
