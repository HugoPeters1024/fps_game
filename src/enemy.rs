use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    assets::{GameAssets, GameState},
    player::Player,
};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_spawn, on_animation_ready, enemy_follows_player).run_if(in_state(GameState::Next)),
        );
    }
}

fn on_spawn(mut commands: Commands, q: Query<Entity, Added<Enemy>>, assets: Res<GameAssets>) {
    for entity in q.iter() {
        commands.entity(entity).insert((
            RigidBody::Dynamic,
            Collider::capsule_y(0.8, 0.3),
            LockedAxes::ROTATION_LOCKED,
            InheritedVisibility::default(),
            children![(
                SceneRoot(assets.cesium_man.clone()),
                Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(0.0, -1.0, 0.0)),
            )],
        ));
    }
}

fn on_animation_ready(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    for (entity, mut player) in &mut players {
        let (graph, node_index) = AnimationGraph::from_clip(assets.cesium_man_animation.clone());
        let graph = graphs.add(graph);
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, node_index, Duration::ZERO)
            .set_speed(2.0)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(graph.clone()))
            .insert(transitions);
    }
}

fn enemy_follows_player(
    player: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    gt: Query<&GlobalTransform>,
    mut t: Query<&mut Transform>,
) {
    let Ok(player) = player.single() else { return };
    let Ok(player_transform) = gt.get(player) else {
        return;
    };

    for enemy in enemies.iter() {
        let Ok(mut enemy_transform) = t.get_mut(enemy) else {
            return;
        };
        let to_player = (player_transform.translation() - enemy_transform.translation)
            .xz()
            .normalize();
        let current_dir = (enemy_transform.rotation * Vec3::Z).xz().normalize();
        let final_rotation = Quat::from_rotation_arc(
            Vec3::new(current_dir.x, 0.0, current_dir.y),
            Vec3::new(to_player.x, 0.0, to_player.y),
        );
        enemy_transform.rotation = enemy_transform
            .rotation
            .slerp(enemy_transform.rotation * final_rotation, 0.03);
        enemy_transform.translation += Vec3::new(to_player.x, 0.0, to_player.y) * 0.05;
    }
}
