use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{Read, System, SystemData, ReadStorage, Join, Entities, WriteStorage};
use amethyst::input::{StringBindings, InputHandler};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst::core::math::{Point3, Vector2, Vector3};
use amethyst::window::ScreenDimensions;
use amethyst::shred::{ReadExpect, Write};
use amethyst::renderer::resources::Tint;
use amethyst::renderer::palette::Srgba;
use crate::TextureCache;
use crate::systems::location::Location;
use crate::systems::logistics::LogisticsNode;
use crate::prefabs::{PrefabCache, NodePrefab};

#[derive(SystemDesc, Default)]
pub struct TestInputSystem {
  saw_mouse_down: bool,
}

impl<'s> System<'s> for TestInputSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Location>,
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, NodePrefab>,
    ReadStorage<'s, Camera>,
    Read<'s, PrefabCache>,
    ReadExpect<'s, ScreenDimensions>,
    Read<'s, InputHandler<StringBindings>>,
    Read<'s, TextureCache>
  );

  fn run(&mut self, (
    entities,
    mut transforms,
    mut locations,
    mut sprites,
    mut prefabs,
    cameras,
    prefab_cache,
    screen,
    input,
    textures): Self::SystemData) {
    let mouse_down = input.action_is_down("select").unwrap_or(false);
    if mouse_down && !self.saw_mouse_down {
      self.saw_mouse_down = true;
      if let Some((x, y)) = input.mouse_position() {
        let (camera, transform) = (&cameras, &transforms).join().next().unwrap();
        let world_pos = camera.screen_to_world_point(
          Point3::new(x, y, 0.0),
          Vector2::new(screen.width(), screen.height()),
          transform
        );

        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(0.1, 0.1, 0.1));
        let mut location = Location::new(world_pos.x, world_pos.y);
        entities
          .build_entity()
          .with(transform, &mut transforms)
          .with(location, &mut locations)
          .with(prefab_cache.get("test_node"), &mut prefabs)
          .with(
            SpriteRender::new(textures.cache.get("circle").unwrap().clone(), 0),
            &mut sprites
          )
          .build();
      }
    } else if !mouse_down {
      self.saw_mouse_down = false;
    }
  }
}