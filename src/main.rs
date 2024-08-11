use bevy::{
    color::palettes::basic::PURPLE, ecs::world::Command, prelude::*, sprite::MaterialMesh2dBundle,
};

fn hello_world() {
    println!("hello Planet Terra!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_persons(mut commands: Commands) {
    commands.spawn((Person, Name("Jeffry Dommer".to_string())));
    commands.spawn((Person, Name("Alexia Dommer".to_string())));
    commands.spawn((Person, Name("Freddy Dommer".to_string())));
}

fn greet_persons(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0);
    }
}

fn update_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alexia Dommer".to_string() {
            name.0 = "remmoD aixelA".to_string();
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_persons, setup));
        app.add_systems(Update, (hello_world, (update_name, greet_persons).chain()));
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::from(PURPLE)),
        ..default()
    });
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
