use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{Read, System, SystemData, ReadStorage, Join, Entities, WriteStorage};
use amethyst::input::{StringBindings, InputHandler};
use amethyst::renderer::{ActiveCamera, Camera, SpriteRender};
use amethyst::core::math::{Point3, Vector2};
use amethyst::window::ScreenDimensions;
use amethyst::shred::ReadExpect;
use crate::TextureCache;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::palette::Srgba;

#[derive(SystemDesc, Default)]
pub struct TestInputSystem {
  saw_mouse_down: bool,
}

impl<'s> System<'s> for TestInputSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, Tint>,
    ReadStorage<'s, Camera>,
    ReadExpect<'s, ScreenDimensions>,
    Read<'s, InputHandler<StringBindings>>,
    Read<'s, TextureCache>
  );

  fn run(&mut self, (entities, mut transforms, mut sprites, mut tints, cameras, screen, input, textures): Self::SystemData) {
    let mouse_down = input.action_is_down("select").unwrap_or(false);
    if mouse_down && !self.saw_mouse_down {
      self.saw_mouse_down = true;
      if let Some((x, y)) = input.mouse_position() {
        let (camera, transform) = (&cameras, &transforms).join().next().unwrap();
        let mouse_position = camera.screen_to_world_point(
          Point3::new(x, y, 0.0),
          Vector2::new(screen.width(), screen.height()),
          transform
        );

        let mut transform = Transform::default();
        transform.set_translation_xyz(mouse_position.x, mouse_position.y, 0.0);
        entities
          .build_entity()
          .with(transform, &mut transforms)
          .with(SpriteRender::new(textures.cache.get("circle").unwrap().clone(), 0), &mut sprites)
          .with(Tint(Srgba::new(1.0, 0.0, 0.0, 1.0)), &mut tints)
          .build();
        println!("make entitiy at {}", mouse_position);
      }
    } else if !mouse_down {
      self.saw_mouse_down = false;
    }
  }
}