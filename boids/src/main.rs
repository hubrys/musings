use amethyst::{
  prelude::*,
  core::TransformBundle,
  input::{InputBundle, StringBindings},
  renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
  },
  utils::application_root_dir
};
use crate::config::FlockConfig;

mod boids;
mod components;
mod systems;
mod config;
mod types;
mod utils;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());
  let app_root = application_root_dir()?;
  let display_config_path = app_root.join("config").join("display.ron");
  let input_config = app_root.join("config").join("bindings.ron");

  let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(input_config)?;

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
    .with_system_desc(systems::DirectBoidsSystems, "direct_boids", &[])
    .with_system_desc(systems::MoveBoidsSystem, "move_boids", &["direct_boids"]);

  let assets_dir = app_root.join("assets");
  let flock_config = FlockConfig::load(app_root.join("config/flock.ron")).unwrap();
  // world.insert(flock_config);
  let mut game = Application::build(assets_dir, boids::Flock::default())?
    .with_resource(flock_config)
    .build(game_data)?;
  game.run();
  Ok(())
}
