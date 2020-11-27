use amethyst::{SimpleState, GameData, StateData, SimpleTrans, Trans};
use amethyst::assets::{ProgressCounter, Handle, PrefabLoader, RonFormat};
use amethyst::core::ecs::WorldExt;
use crate::{utils, TextureCache};
use amethyst::renderer::SpriteSheet;
use std::collections::HashMap;
use crate::prefabs::{PrefabCache, NodePrefab, NodePrefabData};
use amethyst::shred::Write;

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

    world.exec(|mut texture_cache: Write<TextureCache>| {
      texture_cache.cache.insert("circle".to_string(), sprite_sheet);
    });

    world.exec(|data: (Write<PrefabCache>, PrefabLoader<NodePrefabData>)| {
      let (mut cache, loader) = data;
      let handle = loader.load(
        "prefabs/test_node.ron",
        RonFormat,
        &mut progress
      );
      cache.insert("test_node", handle);
    });

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
