use bevy::prelude::*;
use bevy::transform::TransformSystem;
use impacted::CollisionShape;

pub const UPDATE_COLLISION_SHAPES: &str = "update_collision_shapes";

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.translation += vel.0.extend(0.0) * time.delta_seconds();
    }
}

fn update_shape_transforms(
    mut shapes: Query<(&mut CollisionShape, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (mut shape, transform) in shapes.iter_mut() {
        shape.set_transform(*transform);
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position).add_system_to_stage(
            CoreStage::PostUpdate,
            update_shape_transforms
                .label(UPDATE_COLLISION_SHAPES)
                .after(TransformSystem::TransformPropagate),
        );
    }
}
