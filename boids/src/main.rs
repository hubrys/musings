extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate vecmath;

mod math;
mod boid;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs};
use piston::window::WindowSettings;
use piston::{Button, Key, PressEvent, MouseButton, MouseCursorEvent, ReleaseEvent};
use graphics::types::Rectangle;
use graphics::types::Polygon;

use boid::Boid;

pub struct App {
  boid: Boid
}

const SCALE: f64 = 20.0;
const BOID_RENDER: [[f64; 2]; 3]= [
  [-SCALE / 2.0, -SCALE],
  [0.0, SCALE],
  [SCALE / 2.0, SCALE]
];

impl App {
  fn new() -> App {
    App {
      boid: Boid::new()
    }
  }

  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];


    gl.draw(args.viewport(), |c, gl| {
      graphics::clear(GREEN, gl);
      graphics::polygon(
        RED,
        &BOID_RENDER,
        c.transform,
        gl,
      );
    });
  }
}

fn main() {
  let opengl = OpenGL::V3_2;
  let (width, height) = (1080, 720);
  let mut window: Window = WindowSettings::new("boids", [width, height])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut gl = GlGraphics::new(opengl);
  let mut app = App::new();

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      app.render(&mut gl, &args);
    }
  }
}
