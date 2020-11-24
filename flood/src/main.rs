use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, Application};
use amethyst::renderer::{RenderToWindow, RenderFlat2D, RenderingBundle, SpriteSheet};
use amethyst::renderer::types::DefaultBackend;
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::assets::Handle;
use std::collections::HashMap;

mod systems;
mod states;
mod utils;

#[derive(Default)]
pub struct TextureCache {
  cache: HashMap<String, Handle<SpriteSheet>>
}

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());
  let app_root = application_root_dir()?;
  let display_config_path = app_root.join("config").join("display.ron");
  let bindings_config_path = app_root.join("config").join("bindings.ron");
  let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(bindings_config_path)?;


  let game_data = GameDataBuilder::default()
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.0, 0.0, 0.0, 1.0])
        )
        .with_plugin(RenderFlat2D::default())
    )?
    .with_bundle(TransformBundle::new())?
    .with_bundle(input_bundle)?
    .with_system_desc(
      systems::input::TestInputSystem::default(),
      "test_input",
      &["input_system"]
    )
    .with_system_desc(
      systems::location::SyncTransformWithLocationSystem,
      "sync_transform_with_location",
      &[]);

  let assets_dir = app_root.join("assets");
  let mut game = Application::build(assets_dir, states::loading::Loading::default())?
    .build(game_data)?;
  game.run();
  Ok(())
}
