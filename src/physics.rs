use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.translation += vel.0.extend(0.0) * time.delta_seconds();
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position);
    }
}
