mod assets;
mod enemy;
mod extra;
mod player;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{core_pipeline::motion_blur::MotionBlur, math::Affine2, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;
use enemy::{Enemy, EnemyPlugin};

use crate::{
    assets::{GameAssetPlugin, GameAssets, GameState},
    extra::ExtraPlugins,
    player::{Player, PlayerPlugin},
};

const CORNFLOWER_BLUE: Color = Color::linear_rgb(0.392, 0.584, 92.9);

#[derive(Component)]
struct ViewTarget;

#[derive(Component)]
struct ControlViewTarget {
    target: Entity,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "float_me_pls".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FpsControllerPlugin)
        .add_plugins(EditorPlugin::default())
        .add_plugins((GameAssetPlugin, PlayerPlugin, EnemyPlugin, ExtraPlugins))
        .insert_resource(ClearColor(CORNFLOWER_BLUE))
        .add_systems(OnEnter(GameState::Next), startup)
        .add_systems(PostUpdate, (test).run_if(in_state(GameState::Next)))
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    let file = File::open("./level.txt").unwrap();
    let reader = BufReader::new(file);
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            for z in 0..=c.to_digit(10).unwrap() {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(assets.tiles.clone()),
                        uv_transform: Affine2::from_scale(Vec2::splat(1.0)),
                        ..default()
                    })),
                    Transform::from_translation(Vec3::new(x as f32, z as f32, y as f32)),
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                ));
            }
        }
    }

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(7.0, 5.0, 7.0),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        ColliderMassProperties::Mass(400.0),
        Friction {
            coefficient: 0.99,
            combine_rule: CoefficientCombineRule::Max,
        },
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 1_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    let player = commands
        .spawn((
            Player,
            Transform::from_translation(Vec3::new(0.0, 2.6, 0.0)),
            ColliderMassProperties::Mass(80.0),
        ))
        .id();

    let view_target = commands
        .spawn((
            ViewTarget,
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(materials.add(Color::srgb_u8(200, 10, 30))),
        ))
        .id();

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(-0.05, 0.1, 0.3).looking_at(Vec3::new(0.0, 0.1, 0.0), Vec3::Y),
        RenderPlayer {
            logical_entity: player,
        },
        MotionBlur {
            shutter_angle: 0.6,
            samples: 2,
        },
        ControlViewTarget {
            target: view_target,
        },
    ));

    commands.spawn((Transform::from_xyz(5.0, 2.0, 5.0), Enemy));
    commands.spawn((Transform::from_xyz(10.0, 2.0, 5.0), Enemy));
}

fn test(
    fps_controller: Query<Entity, With<FpsController>>,
    source: Query<(&GlobalTransform, &ControlViewTarget)>,
    mut target: Query<&mut Transform, With<ViewTarget>>,
    rapier_context: ReadRapierContext,
) {
    for (gt, target_entity) in source.iter() {
        if let Ok(mut target) = target.get_mut(target_entity.target) {
            let direction = gt.rotation() * Vec3::NEG_Z;
            let origin = gt.translation();
            let predicate = |e| !fps_controller.contains(e);
            let filter = QueryFilter::new().predicate(&predicate);
            if let Some((_, dist)) = rapier_context
                .single()
                .unwrap()
                .cast_ray(origin, direction, 100.0, true, filter)
            {
                target.translation = origin + direction * dist;
            }
        }
    }
}
