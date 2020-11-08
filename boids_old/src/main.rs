extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
mod math;
mod boid;
mod camera;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs};
use piston::window::WindowSettings;
use piston::{Button, Key, PressEvent, MouseButton, MouseCursorEvent, ReleaseEvent};
use graphics::types::Rectangle;
use graphics::types::Polygon;
use graphics::Context;

use boid::Boid;
use camera::Camera;
use math::{matrix, Matrix2d};

pub struct App {
  camera: Camera,
  boid: Boid,
  boid2: Boid
}

const SCALE: f64 = 10.0;
const BOID_RENDER: [[f64; 2]; 3]= [
  [-SCALE, -SCALE],
  [0.0, SCALE],
  [SCALE, -SCALE]
];

impl App {
  fn new() -> App {
    App {
      camera: Camera::new(5.0),
      boid: Boid::new(),
      boid2: Boid::new()
    }
  }

  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
    const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    gl.draw(args.viewport(), |c, gl| {
      graphics::clear(GREEN, gl);
      let root_transform =
        matrix::multiply(
          c.transform,
          self.camera.gen_transform(c.get_view_size())
        );
      render_boid(gl, root_transform, &self.boid);
      // render_boid(gl, root_transform, &self.boid2);
    });
  }
}

fn render_boid(gl: &mut GlGraphics, transform: Matrix2d, boid: &Boid) {
  const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
  use matrix::{multiply, translate, scale};

  graphics::polygon(
    RED,
    &BOID_RENDER,
    multiply(transform, translate(boid.pos)),
    gl,
  );
}

fn main() {
  let opengl = OpenGL::V3_2;
  let (width, height) = (400, 300);
  let mut window: Window = WindowSettings::new("boids_old", [width, height])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut gl = GlGraphics::new(opengl);
  let mut app = App::new();
  app.boid2.pos = [200.0, 150.0];

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      app.render(&mut gl, &args);
    }
  }
}
