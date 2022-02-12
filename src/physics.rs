use bevy::prelude::*;
use bevy::transform::TransformSystem;
use impacted::CollisionShape;

use crate::GameState;

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

// Marks entity as fixed in relation to the camera
#[derive(Component)]
pub struct FixedOffset(pub Vec2);

fn update_fixed_position(
    camera: Query<&Transform, (With<Camera>, With<Velocity>)>,
    mut fixed_entities: Query<(&mut Transform, &FixedOffset), Without<Camera>>,
) {
    if camera.is_empty() {
        return;
    }
    let camera_transform = camera.single();
    for (mut t, offset) in fixed_entities.iter_mut() {
        t.translation = camera_transform.translation + offset.0.extend(0.0);
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_position)
                .with_system(update_fixed_position),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            update_shape_transforms
                .label(UPDATE_COLLISION_SHAPES)
                .after(TransformSystem::TransformPropagate),
        );
    }
}
