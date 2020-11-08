use crate::math::{Matrix2d, Vector2};
use crate::math::matrix;
use graphics::math::multiply;

pub struct Camera {
  size: f64
}

impl Camera {
  /// Create new camera with given size
  pub fn new(size: f64) -> Camera {
    Camera {
      size
    }
  }

  /// Generate root transform for current camera settings and windowheight
  pub fn gen_transform(&self, window_dims: [f64; 2]) -> Matrix2d {
    // v = viewport
    let v_height = self.size * 2.0;

    matrix::multiply(
      matrix::scale(1.0, 1.0),// window_dims[1] / v_height),
      matrix::multiply(
        matrix::translate([
          window_dims[0] / 2.0,
          window_dims[1] / 2.0
        ]),
        // matrix::translate([
        //   .0, 0.0
        // ]),
        matrix::scale(1.0, 1.0)
      )
    )
  }
}