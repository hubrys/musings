use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::derive::SystemDesc;
use amethyst::core::math::{Vector2};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::ecs::Entities;
use crate::components::{BoidIntent, Boid};
use crate::config::FlockConfig;
use crate::systems::boid_rules::base_rules::{SeparationRule, CohesionRule, AlignmentRule};
use crate::systems::BoidRule;

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
      let mut cohesion = CohesionRule::new(boid.position, flock.cohesion_distance, flock.cohesion_weight);
      let mut separation = SeparationRule::new(flock.separation_distance, flock.separation_weight);
      let mut alignment = AlignmentRule::new(flock.alignment_distance, flock.alignment_weight);

      for (other_ent, other_boid) in (&ents, &boids).join() {
        // Don't include self in calculations
        if ent.id() == other_ent.id() {
          continue;
        }

        let boid_separation = distance(&boid.position, &other_boid.position);
        cohesion.process_boid(boid, other_boid, boid_separation);
        separation.process_boid(boid, other_boid, boid_separation);
        alignment.process_boid(boid, other_boid, boid_separation);
      }
      boid_intent.force =
        cohesion.applied_force() +
          separation.applied_force() +
          alignment.applied_force();
    }
  }
}

fn distance(lhs: &Vector2<f32>, rhs: &Vector2<f32>) -> f32 {
  (lhs - rhs).norm()
}
