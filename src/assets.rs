use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "zenk_rzmk-357.glb#Scene0")]
    pub gun: Handle<Scene>,

    #[asset(path = "tiles.png")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
    pub tiles: Handle<Image>,

    #[asset(path = "Cesium_Man.glb#Animation0")]
    pub cesium_man_animation: Handle<AnimationClip>,

    #[asset(path = "Cesium_Man.glb#Scene0")]
    pub cesium_man: Handle<Scene>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Next,
}

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Next)
                .load_collection::<GameAssets>(),
        );
    }
}
