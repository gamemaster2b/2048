use bevy::prelude::*;
use itertools::Itertools;
mod colors;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 120.0;
const TILE_SPACER: f32 = 15.0;

#[derive(Component)]
struct Board {
    size: u8,
}

fn spawn_board(mut commands: Commands) {
    let board = Board { size: 4 };
    let physical_board_size = f32::from(board.size) * TILE_SIZE;
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(physical_board_size, physical_board_size)),
                color: colors::BOARD,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            let offset = -physical_board_size / 2.0 + 0.5 * TILE_SIZE;

            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(
                            TILE_SIZE - TILE_SPACER * 2f32,
                            TILE_SIZE - TILE_SPACER * 2f32,
                        )),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        offset + f32::from(tile.0) * TILE_SIZE,
                        offset + f32::from(tile.1) * TILE_SIZE,
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(colors::CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup, spawn_board))
        .run();
}
