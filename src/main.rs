use bevy::{input::prelude::ButtonInput, prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use std::{thread, time::Duration};

const SNAKE_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const SNAKE_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SNAKE_SIZE: f32 = 20.;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const GRID_X: u32 = (WINDOW_HEIGHT as f32 / SNAKE_SIZE) as u32;
const GRID_Y: u32 = (WINDOW_WIDTH as f32 / SNAKE_SIZE) as u32;

#[derive(Component)]
struct SnakeHead;
#[derive(Component)]
struct Food;
#[derive(Resource)]
struct FoodGrid(u32, u32);
#[derive(Resource)]
struct SnakeHeading(u8);

fn main() {
    let mut app = App::new();

    app.add_systems(Startup, setup);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            title: "Snake Game".to_string(),
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Update, snake_movement);

    app.add_systems(Update, change_heading);

    app.insert_resource(SnakeHeading(0));

    app.add_systems(Update, sleep);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn snake head
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(SNAKE_SIZE, SNAKE_SIZE)).into(),
            material: materials.add(SNAKE_COLOR),
            transform: Transform::from_translation(SNAKE_STARTING_POSITION),
            ..default()
        })
        .insert(SnakeHead);

    // Spawn food
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(SNAKE_SIZE, SNAKE_SIZE)).into(),
            material: materials.add(FOOD_COLOR),

            transform: Transform::from_translation(SNAKE_STARTING_POSITION),
            ..default()
        })
        .insert(Food);
}

fn snake_movement(
    mut head_positions: Query<(&SnakeHead, &mut Transform)>,
    heading: Res<SnakeHeading>,
) {
    for (_head, mut transform) in head_positions.iter_mut() {
        match heading.0 {
            0 => transform.translation.y += 20.,
            1 => transform.translation.y -= 20.,
            2 => transform.translation.x += 20.,
            3 => transform.translation.x -= 20.,
            _ => {}
        }
    }
}

fn change_heading(mut heading: ResMut<SnakeHeading>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::ArrowUp) {
        heading.0 = 0;
    } else if key.just_pressed(KeyCode::ArrowDown) {
        heading.0 = 1;
    } else if key.just_pressed(KeyCode::ArrowRight) {
        heading.0 = 2;
    } else if key.just_pressed(KeyCode::ArrowLeft) {
        heading.0 = 3;
    }
}

fn sleep() {
    thread::sleep(Duration::from_millis(300));
}

fn assign_food_location(mut food_grid: ResMut<FoodGrid>, food: Res<Food>) {
    food_grid.0 = rand::thread_rng().gen_range(0..=GRID_X);
    food_grid.1 = rand::thread_rng().gen_range(0..=GRID_Y);

    let grid_middle_number_x = GRID_X / 2;
    let grid_middle_number_y = GRID_Y / 2;

    if grid_middle_number_x > food_grid.0 {
        println!(food_grid)
    } else {
        println!(food_grid)
    }
}

