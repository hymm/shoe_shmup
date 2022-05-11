use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin).add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_audio),
        );
    }
}

fn start_audio(audio: Res<Audio>) {
    audio.set_volume(0.5);
}
