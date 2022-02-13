#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod bullet;
mod constants;
mod enemy;
mod loading;
mod menu;
mod pause_menu;
mod physics;
mod player;
mod player_rail;
mod serialize;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::bullet::BulletPlugin;
pub use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::enemy::EnemyPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::pause_menu::PauseMenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::serialize::SerializePlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, diagnostic::EntityCountDiagnosticsPlugin};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    LoadingAssets,
    // load level from scene file
    LoadLevel,
    // massage data loaded from a scene file
    PostLoadLevel,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    Paused,
    PlayerDead,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::LoadingAssets)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(SerializePlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(EntityCountDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
