use amethyst::{SimpleState, GameData, StateData, SimpleTrans, Trans};
use amethyst::assets::{Cache, ProgressCounter, Loader, Handle};
use amethyst::core::ecs::WorldExt;
use crate::{utils, TextureCache};
use amethyst::renderer::SpriteSheet;
use std::collections::HashMap;

#[derive(Default)]
pub struct Loading {
  progress: Option<ProgressCounter>,
  cache: Option<HashMap<String, Handle<SpriteSheet>>>
}

impl SimpleState for Loading {
  fn on_start(&mut self, data: StateData<GameData>) {
    let mut cache = HashMap::new();
    let mut progress = ProgressCounter::new();
    let mut world = data.world;
    let sprite_sheet = utils::load_sprite_sheet(world, "textures/circle", &mut progress);
    let mut texture_cache = world.fetch_mut::<TextureCache>();
    cache.insert("circle".to_string(), sprite_sheet.clone());
    texture_cache.cache.insert("circle".to_string(), sprite_sheet);
    self.cache = Some(cache);
    self.progress = Some(progress);
  }

  fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
    if let Some(progress) = &self.progress {
      println!("LOADED");
      Trans::Switch(Box::new(
        super::test::Test {
          cache: self.cache.take().unwrap()
        }
      ))
    } else {
      Trans::None
    }
  }
}
