use bevy::prelude::*;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use uwuifier::uwuify_str_sse;
mod colors;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 120.0;
const TILE_SPACER: f32 = 15.0;

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Board {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn tile_position_on_board(&self, x_axis: u8, y_axis: u8, z_axis: f32) -> (f32, f32, f32) {
        let offset = -&self.physical_size / 2.0 + 0.5 * TILE_SIZE;
        (
            (offset + TILE_SIZE * f32::from(x_axis) + f32::from(x_axis + 1) * TILE_SPACER),
            (offset + TILE_SIZE * f32::from(y_axis) + f32::from(y_axis + 1) * TILE_SPACER),
            z_axis,
        )
    }
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
                color: colors::BOARD,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                let (x, y, z) = board.tile_position_on_board(tile.0, tile.1, 1.0);

                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x, y, z),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}

#[derive(Component)]
struct Points {
    value: usize,
}

#[derive(Component)]
struct Position {
    x_axis: u8,
    y_axis: u8,
}

#[derive(Component)]
struct TileText;

fn spawn_tiles(mut comands: Commands, query_board: Query<&Board>) {
    let board = query_board.single();
    let mut rng = rand::thread_rng();
    let starting_tiles = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);
    for (pos_x, pos_y) in starting_tiles.iter() {
        let pos = Position {
            x_axis: *pos_x,
            y_axis: *pos_y,
        };
        let points = Points { value: 2usize };
        let (x, y, z) = board.tile_position_on_board(pos.x_axis, pos.y_axis, 2.0);
        comands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: colors::TILE,
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.9, TILE_SIZE * 0.9)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn(Text2dBundle {
                        text: Text::from_section(
                            points.value.to_string(),
                            TextStyle {
                                font_size: TILE_SIZE * 0.9,
                                color: colors::TEXT,
                                ..Default::default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..Default::default()
                    })
                    .insert(TileText);
            })
            .insert(points)
            .insert(pos);
    }
}

fn main() {
    println!("{}", uwuify_str_sse("hello world"));
    App::new()
        .insert_resource(ClearColor(colors::CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(
            Startup,
            (setup, (spawn_board, apply_deferred, spawn_tiles).chain()),
        )
        .run();
}
