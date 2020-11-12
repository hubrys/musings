use amethyst::core::math::Vector2;
use crate::systems::BoidRule;
use crate::components::Boid;

pub struct CohesionRule {
  boid_position: Vector2<f32>,
  accumulator: Vector2<f32>,
  count: u32,
  weight: f32,
}

impl CohesionRule {
  pub fn new(boid_position: Vector2<f32>, weight: f32) -> Self {
    CohesionRule {
      boid_position,
      accumulator: Vector2::zeros(),
      count: 0,
      weight
    }
  }
}

impl BoidRule for CohesionRule {
  fn process_boid(&mut self, boid: &Boid, other_boid: &Boid, separation: f32) {
    self.count += 1;
    self.accumulator += other_boid.position;
  }

  fn applied_force(&self) -> Vector2<f32> {
    if self.count > 0 {
      ((self.accumulator / self.count as f32) - self.boid_position) * self.weight
    } else {
      Vector2::zeros()
    }
  }
}

pub struct SeparationRule {
  accumulator: Vector2<f32>,
  separation_distance: f32,
  weight: f32,
}

impl SeparationRule {
  pub fn new(separation_distance: f32, weight: f32) -> Self {
    SeparationRule {
      accumulator: Vector2::zeros(),
      separation_distance,
      weight
    }
  }
}

impl BoidRule for SeparationRule {
  fn process_boid(&mut self, boid: &Boid, other_boid: &Boid, separation: f32) {
    if separation < self.separation_distance {
      let vector = boid.position - other_boid.position;
      let distance = vector.norm();
      self.accumulator += vector / (distance / self.separation_distance);
      // self.accumulator += boid.position - other_boid.position
    }
  }

  fn applied_force(&self) -> Vector2<f32> {
    self.accumulator * self.weight
  }
}

pub struct AlignmentRule {
  accumulator: Vector2<f32>,
  count: u32,
  weight: f32,
}

impl AlignmentRule {
  pub fn new(weight: f32) -> Self {
    AlignmentRule {
      weight,
      accumulator: Vector2::zeros(),
      count: 0,
    }
  }
}

impl BoidRule for AlignmentRule {
  fn process_boid(&mut self, boid: &Boid, other_boid: &Boid, separation: f32) {
    self.count += 1;
    self.accumulator += other_boid.velocity;
  }

  fn applied_force(&self) -> Vector2<f32> {
    if self.count > 0 {
      self.accumulator / self.count as f32 * self.weight
    } else {
      self.accumulator
    }
  }
}

pub struct BoundaryRule;

impl BoundaryRule {
  pub fn create_offset_boundary(arena_size: [f32; 2], offset: f32) -> [f32; 4] {
    let [width, height] = arena_size;
    [
      offset,
      height - offset,
      width - offset,
      offset
    ]
  }

  pub fn applied_force(boundary: [f32; 4], force: f32, boid: &Boid) -> Vector2<f32> {
    let [left, top, right, bottom] = boundary;
    let position = boid.position;
    let x = {
      if position.x < left {
        left - position.x
      } else if position.x > right {
        right - position.x
      } else {
        0.0
      }
    };

    let y = {
      if position.y < bottom {
        bottom - position.y
      } else if position.y > top {
        top - position.y
      } else {
        0.0
      }
    };

    Vector2::new(x, y) * force
  }
}