use bevy::prelude::*;
mod colors;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 120.0;

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
        .insert(board);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Oklcha(Oklcha {
            lightness: 0.2706,
            chroma: 0.035,
            hue: 267.69,
            alpha: 1.0,
        })))
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
