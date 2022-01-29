use crate::GameState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn spawn_enemy(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(30.0, 30.0),
        origin: shapes::RectangleOrigin::Center,
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(Color::rgb_u8(164, 69, 55))),
        Transform::from_xyz(-20.0, 200.0, 1.0),
    ));
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy));
    }
}
