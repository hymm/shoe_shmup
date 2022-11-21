use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::LoadingAssets)
                .continue_to_state(GameState::Menu)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(Resource, AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/shoot.wav")]
    pub shoot: Handle<AudioSource>,
    #[asset(path = "audio/empty_clip.wav")]
    pub empty_clip: Handle<AudioSource>,
    #[asset(path = "audio/reload.wav")]
    pub reload: Handle<AudioSource>,
    #[asset(path = "audio/explode.wav")]
    pub explode: Handle<AudioSource>,
    #[asset(path = "audio/player_death.wav")]
    pub player_death: Handle<AudioSource>,
}

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}
