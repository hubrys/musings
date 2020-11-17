use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::derive::SystemDesc;
use amethyst::core::math::{Vector2};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::ecs::{Entities, ParJoin, BitSet};
use crate::components::{Forces, Movement, Boid, Enemy, SpacePointer};
use crate::config::FlockConfig;
use crate::systems::boid_rules::base_rules::{SeparationRule, CohesionRule, AlignmentRule, BoundaryRule};
use crate::systems::BoidRule;
use amethyst::core::ecs::rayon::iter::ParallelIterator;
use crate::space_partition::TiledSpace;

/// Moves boids based on simple rules
/// rules:
/// - Separation - avoid crowding neighbors
/// - Alignment - go in the same direction as neighbors
/// - Cohesion - move closer to center mass of local neighbors
#[derive(SystemDesc)]
pub struct DirectBoidsSystems {
  targets: BitSet
}

impl DirectBoidsSystems {
  pub fn new() -> Self {
    DirectBoidsSystems {
      targets: BitSet::new()
    }
  }
}

impl<'s> System<'s> for DirectBoidsSystems {
  type SystemData = (
    Read<'s, InputHandler<StringBindings>>,
    Read<'s, FlockConfig>,
    Entities<'s>,
    ReadStorage<'s, Movement>,
    WriteStorage<'s, Forces>,
    ReadStorage<'s, Boid>,
    ReadStorage<'s, Enemy>,
    Read<'s, TiledSpace<u32>>
  );

  fn run(&mut self, (
    _input,
    flock,
    ents,
    movements,
    mut forces,
    boids,
    enemies,
    grid
  ): Self::SystemData) {
    let boundary = BoundaryRule::create_offset_boundary(
      flock.arena_size, flock.boundary_offset);
    // for (ent, movement, force, boid) in
    (&ents, &movements, &mut forces, &boids)
      // .par_join()
      .par_join()
      .for_each(|(ent, movement, force, _)| {
        let mut cohesion = CohesionRule::new(movement.position, flock.cohesion_weight);
        let mut separation = SeparationRule::new(flock.separation_distance, flock.separation_weight);
        let mut enemy_separation = SeparationRule::new(flock.enemy_separation_distance, flock.enemy_separation_weight);
        let mut alignment = AlignmentRule::new(flock.alignment_weight);

        let pos = movement.position;
        let mut close_ents = BitSet::new();
        let mut count = 0;
        if ent.id() == 0 {
          grid.get_elems([
            pos.x - flock.separation_distance,
            pos.y - flock.separation_distance,
            pos.x + flock.separation_distance,
            pos.y + flock.separation_distance
          ])
            .for_each(|node| {
              count += 1;
              close_ents.add(node);
            });
        }

        (&ents, &movements, boids.maybe(), enemies.maybe())
          .join()
          .for_each(|(other_ent, other_movement, maybe_boid, maybe_enemy)| {
            // Don't include self in calculations
            if ent.id() == other_ent.id() {
              return;
            }

            let boid_separation = distance(&movement.position, &other_movement.position);
            if boid_separation < flock.local_group_distance {
              if let Some(_b) = maybe_boid {
                cohesion.process_boid(movement, other_movement, boid_separation);
                separation.process_boid(movement, other_movement, boid_separation);
                alignment.process_boid(movement, other_movement, boid_separation);
              }

              if let Some(_e) = maybe_enemy {
                enemy_separation.process_boid(movement, other_movement, boid_separation);
              }
            }
          });

        force.force =
          BoundaryRule::applied_force(boundary, flock.boundary_force, movement) +
            cohesion.applied_force() +
            separation.applied_force() +
            enemy_separation.applied_force() +
            alignment.applied_force();
      });

    for (movement, force, _) in (&movements, &mut forces, &enemies).join() {
      force.force = BoundaryRule::applied_force(boundary, flock.boundary_force, movement);
    }
  }
}

fn distance(lhs: &Vector2<f32>, rhs: &Vector2<f32>) -> f32 {
  (lhs - rhs).norm()
}
