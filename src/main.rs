use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(FixedUpdate, snake_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<SnakeHead>>,
) {
    let mut transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 2.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 2.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 2.;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 2.;
    }
}
