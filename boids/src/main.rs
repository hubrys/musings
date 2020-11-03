extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs};
use piston::window::WindowSettings;
use piston::{Button, Key, PressEvent, MouseButton, MouseCursorEvent, ReleaseEvent};
use graphics::types::Rectangle;

pub struct App {
}

impl App {
  fn new() -> App {
    App {}
  }

  fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    gl.draw(args.viewport(), |c, gl| {
      graphics::clear(GREEN, gl);
    });
  }
}

fn main() {
  let opengl = OpenGL::V3_2;
  let (width, height) = (1080, 720);
  let mut window: Window = WindowSettings::new("fractal-viewer", [width, height])
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
