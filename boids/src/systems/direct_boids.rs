use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::core::math::{Vector3, Vector2, UnitQuaternion};
use amethyst::input::{InputHandler, StringBindings};
use crate::components::{BoidIntent, TurnDirection, Boid};
use crate::config::FlockConfig;
use amethyst::core::ecs::world::EntitiesRes;
use amethyst::core::ecs::Entities;

const LOCAL_GROUP_RADIUS: f32 = 100.0;

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
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (
    input,
    flock,
    ents,
    boids,
    mut boid_intents,
    mut transforms
  ): Self::SystemData) {
    for (ent, transform, boid, boid_intent) in (&ents, &transforms, &boids, &mut boid_intents).join() {
      /// TODO: Ignore the current boid
      // Center of mass
      let mut com = Vector2::new(0.0, 0.0);
      let mut cohesion_count = 0;
      for (other_ent, other_boid, other_transform) in (&ents, &boids, &transforms).join() {
        let separation = distance(&boid.position, &other_boid.position);
        if separation < flock.cohesion_distance {
          cohesion_count += 1;
          let other_translation = other_transform.translation();
          com.x += other_translation.x;
          com.y += other_translation.y;
        }
      }

      com.x /= cohesion_count as f32;
      com.y /= cohesion_count as f32;

      let com_vec = com - boid.position;
      let com_angle = com_vec.y.atan2(com_vec.x).to_degrees();
      let mut angle_diff = com_angle - boid.rotation;

      // if angle_diff.to_degrees() > 180.0 {
      //   angle_diff = -360.0 + angle_diff;
      // } else if angle_diff.to_degrees() < -180.0 {
      //   angle_diff = 360.0 + angle_diff;
      // }

      // let axis_angle = Vector3::<f32>::z() * com_angle.to_degrees();
      // let com_quat = UnitQuaternion::new(axis_angle);
      // let boid_quat = UnitQuaternion::new(Vector3::<f32>::z() * boid.rotation);
      // let (_, _, result) = com_quat.euler_angles();

      if ent.id() == 0 {
        // println!("com: {}", com);
        println!("angle: {} {}", ent.id(), com_angle);
      }

      boid_intent.turning = if angle_diff < 0.0 { -1.0 } else { 1.0 };

      boid_intent.turning = {
        if input.action_is_down("left").unwrap() {
          1.0
        } else if input.action_is_down("right").unwrap() {
          -1.0
        } else {
          boid_intent.turning
        }
      };
    }
  }
}

fn distance(lhs: &Vector2<f32>, rhs: &Vector2<f32>) -> f32 {
  (lhs - rhs).norm()
}
