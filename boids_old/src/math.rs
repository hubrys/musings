extern crate vecmath;

use graphics::math;
pub use math::Matrix2d as Matrix2d;
pub type Vector2 = vecmath::Vector2<f64>;

pub mod matrix {
  use graphics::math;
  use std::ops;
  use super::Matrix2d;

  pub use math::translate;
  pub use math::multiply;
  pub use math::scale;
}

