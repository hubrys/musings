use amethyst::prelude::*;
use amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans};
use amethyst::assets::{Handle, Loader, AssetStorage};
use amethyst::core::Transform;
use amethyst::core::math::{Vector2, Vector3};
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use amethyst::renderer::{Camera, SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, SpriteRender};
use rand::Rng;
use crate::components::{Forces, Movement, Boid, Enemy, SpacePointer};
use crate::config::FlockConfig;
use crate::space_partition::TiledSpace;

#[derive(Default)]
pub struct Flock {}

impl SimpleState for Flock {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let mut world = data.world;
    let (arena_size, boid_count, enemy_count) = {
      let config = world.read_resource::<FlockConfig>();
      (config.arena_size, config.boid_count, config.enemy_count)
    };

    let sprite_sheet = load_sprite_sheet(world, "textures/triangle");
    let sprite_render = SpriteRender::new(sprite_sheet, 0);
    let mut rng = rand::thread_rng();
    for _ in 0..boid_count {
      let x = rng.gen_range(0.0, arena_size[0]);
      let y = rng.gen_range(0.0, arena_size[1]);
      create_boid(
        world,
        sprite_render.clone(),
        [x, y]
      )
    }

    let enemy_sprite_sheet = load_sprite_sheet(world, "textures/circle");
    let enemy_sprite_render = SpriteRender::new(enemy_sprite_sheet, 0);
    for _ in 0..enemy_count {
      let x = rng.gen_range(0.0, arena_size[0]);
      let y = rng.gen_range(0.0, arena_size[1]);
      create_enemy(
        world,
        enemy_sprite_render.clone(),
        [x, y]
      )
    }
    init_camera(world, arena_size);
  }

  fn handle_event(
    &mut self,
    _data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
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

fn init_camera(world: &mut World, arena_size: [f32; 2]) {
  let [width, height] = arena_size;
  let mut transform = Transform::default();
  transform.set_translation_xyz(width / 2.0, height / 2.0, 1.0);
  world
    .create_entity()
    .with(Camera::standard_2d(width, height))
    .with(transform)
    .build();
}

fn create_boid(
  world: &mut World,
  sprite: SpriteRender,
  location: [f32; 2]) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(location[0], location[1], 0.0);
  transform.set_scale(Vector3::new(0.1, 0.1, 1.0));
  world
    .create_entity()
    .with(sprite)
    .with(transform)
    .with(Boid {})
    .with(Movement {
      position: Vector2::new(location[0], location[1]),
      velocity: Vector2::new(0.0, 0.0),
    })
    .with(SpacePointer::default())
    .with(Forces::default())
    .build();
}

fn create_enemy(
  world: &mut World,
  sprite: SpriteRender,
  location: [f32; 2]) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(location[0], location[1], 0.0);
  transform.set_scale(Vector3::new(0.7, 0.7, 1.0));
  world
    .create_entity()
    .with(sprite)
    .with(transform)
    .with(Enemy {})
    .with(Movement {
      position: Vector2::new(location[0], location[1]),
      velocity: Vector2::new(location[1], location[0]).normalize() * 50.0,
    })
    .with(Forces::default())
    .build();
}

