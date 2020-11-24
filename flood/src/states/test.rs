use amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans};
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use crate::utils;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use std::collections::HashMap;
use amethyst::assets::Handle;
use amethyst::shred::World;
use amethyst::core::math::Vector2;

#[derive(Default)]
pub struct Test {
  pub cache: HashMap<String, Handle<SpriteSheet>>
}

impl SimpleState for Test {
  fn on_start(&mut self, data: StateData<GameData>) {
    let mut world = data.world;
    utils::init_camera(world, [1200.0, 800.0]);
  }

  fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent
  ) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      if is_close_requested(&event) ||
        is_key_down(&event, VirtualKeyCode::Escape) {
        Trans::Quit
      } else {
        Trans::None
      }
    } else {
      Trans::None
    }
  }
}

fn add_node(world: &mut World, location: Vector2<f32>) {}