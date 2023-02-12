use bevy::prelude::*;
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_gltf)
        .add_system(keyboard_animation_control)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(AtmosphereCamera::default());
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 5.0),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}

fn spawn_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    commands.insert_resource(Animations(vec![ass.load("large.glb#Animation0")]));
    let chub_gltf = ass.load("large.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: chub_gltf,
        transform: Transform::from_xyz(0.0, -1.75, -5.0),
        ..Default::default()
    });
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play(animations.0[*current_animation].clone_weak())
                .repeat();
        }
    }
}
