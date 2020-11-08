use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join, Read};
use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::core::math::Vector3;
use amethyst::input::{InputHandler, StringBindings};
use crate::components::{Boid, TurnDirection};

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
    WriteStorage<'s, Boid>,
    WriteStorage<'s, Transform>,
  );

  fn run(&mut self, (input, mut boids, mut transforms): Self::SystemData) {
    for (boid, transform) in (&mut boids, &transforms).join() {
      // let neighbors = (&boids, &transforms).join().filter(|(other_boid, other_transform)| {
      //   let separation = distance_between(transform.translation(), other_transform.translation());
      //   separation < LOCAL_GROUP_RADIUS
      // });
      boid.turn_direction = {
        if input.action_is_down("left").unwrap() { TurnDirection::Left }
        else if input.action_is_down("right").unwrap() { TurnDirection::Right }
        else {TurnDirection::None}
      };
    }
  }
}

fn distance_between(from: &Vector3<f32>, to: &Vector3<f32>) -> f32 {
  0.0
}