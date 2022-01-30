// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use shoe_shmup::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(211, 228, 222)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Shoe Shmup".to_string(), // ToDo
            ..Default::default()
        })
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
