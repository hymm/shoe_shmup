// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::{App, ClearColor, Color, Msaa, PluginGroup, WindowDescriptor};
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use shoe_shmup::{GamePlugin, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(211, 228, 222)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                title: "Shoe Shmup".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
