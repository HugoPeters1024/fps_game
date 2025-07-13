use bevy::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub struct DelayedTransform {
    prev_translation: Option<Transform>,
    translation_alpha: f32,
    rotation_alpha: f32
}

impl Default for DelayedTransform {
    fn default() -> Self {
        DelayedTransform {
            prev_translation: None,
            translation_alpha: 0.0,
            rotation_alpha: 0.75,
        }
    }
}

fn update_delayed_transform(mut q: Query<(&mut Transform, &mut DelayedTransform)>) {
    for (mut current, mut delay) in q.iter_mut() {
        if let Some(previous) = delay.prev_translation {
            current.translation = current.translation.lerp(previous.translation, delay.translation_alpha);
            current.rotation = current.rotation.lerp(previous.rotation, delay.rotation_alpha);
        }

        delay.prev_translation = Some(current.clone());
    }
}


pub struct ExtraPlugins;

impl Plugin for ExtraPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_delayed_transform);
    }
}


