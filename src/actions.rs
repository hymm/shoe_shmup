use crate::{physics::Velocity, GameState};
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label("actions")
                .with_system(set_movement_actions)
                .with_system(set_point_actions)
                .with_system(set_shoot_action),
        );
    }
}

#[derive(Resource, Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub player_point: Option<Vec2>,
    pub player_stop: bool,
    pub player_shoot: bool,
}

fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
) {
    actions.player_stop =
        keyboard_input.pressed(KeyCode::F) || mouse_button.pressed(MouseButton::Right);
}

fn set_point_actions(
    mut actions: ResMut<Actions>,
    mut cursor_pos: EventReader<CursorMoved>,
    camera: Query<&Transform, (With<Camera>, With<Velocity>)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    for position in cursor_pos.iter() {
        let transform = camera.single();
        // convert cursor_pos into world coordinates
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        actions.player_point =
            Some(position.position - size / 2.0 + transform.translation.truncate());
    }
}

fn set_shoot_action(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
) {
    actions.player_shoot = if actions.player_stop {
        keyboard_input.just_pressed(KeyCode::Space) || mouse_button.just_pressed(MouseButton::Left)
    } else {
        false
    };
}
