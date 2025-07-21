use std::f32::consts::{PI, TAU};

use crate::{assets::{GameAssets, GameState}, extra::DelayedTransform};
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
#[require(Transform, InheritedVisibility)]
pub struct Player;

pub struct PlayerPlugin;

#[derive(Component)]
#[require(Transform, InheritedVisibility)]
struct GunWobble;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_spawn.run_if(in_state(GameState::Next)))
            .add_systems(Update, (gun_wobbles, manage_cursor));
    }
}

fn on_spawn(mut commands: Commands, target: Query<Entity, Added<Player>>, assets: Res<GameAssets>) {
    for e in target.iter() {
        commands.entity(e).insert((
            Collider::capsule_y(0.8, 0.5),
            RigidBody::Dynamic,
            Friction {
                coefficient: 0.2,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.1,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController { ..default() },
            CameraConfig {
                height_offset: -0.15,
            },
        ));

        commands.spawn((
            RenderPlayer { logical_entity: e },
            InheritedVisibility::default(),
            Transform::default(),
            DelayedTransform::default(),
            children![(
                InheritedVisibility::default(),
                Transform::from_xyz(0.04, -0.1, -0.3),
                children![(
                    GunWobble,
                    children![(
                        SceneRoot(assets.gun.clone()),
                        Transform::from_rotation(Quat::from_rotation_y(PI))
                    )]
                )]
            )],
        ));
    }
}

fn gun_wobbles(mut q: Query<&mut Transform, With<GunWobble>>, time: Res<Time>) {
    for mut t in q.iter_mut() {
        t.translation.y = ((6.0 * time.elapsed_secs()).sin()) * 0.002;
    }
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    for mut window in &mut window_query {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
        }
        if key.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            for mut controller in &mut controller_query {
                controller.enable_input = false;
            }
        }
    }
}
