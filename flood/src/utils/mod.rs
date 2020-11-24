use amethyst::shred::World;
use amethyst::assets::{Handle, Loader, AssetStorage, Progress, ProgressCounter};
use amethyst::renderer::{SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, Camera};
use amethyst::core::ecs::{WorldExt, Builder, Entity};
use amethyst::core::Transform;

pub fn load_sprite_sheet(world: &mut World, path: &str, progress: &mut ProgressCounter) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      // "textures/triangle"
      path.to_owned() + ".png",
      ImageFormat::default(),
      (),
      &texture_storage
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    path.to_owned() + ".ron",
    SpriteSheetFormat(texture_handle),
    progress,
    &sprite_sheet_store
  )
}

pub fn init_camera(world: &mut World, view_port: [f32; 2]) -> Entity {
  let [width, height] = view_port;
  let mut transform = Transform::default();
  transform.set_translation_xyz(width / 2.0, height / 2.0, 1.0);
  world
    .create_entity()
    .with(Camera::standard_2d(width, height))
    .with(transform)
    .build()
}

