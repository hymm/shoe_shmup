use bevy::prelude::*;
use bevy::transform::TransformSystem;
use impacted::CollisionShape;

use crate::GameState;

#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone)]
pub struct UpdateCollisionShapes;

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
        app.add_systems(
            (update_position, update_fixed_position).in_set(OnUpdate(GameState::Playing)),
        )
        .add_system(
            update_shape_transforms
                .in_base_set(CoreSet::PostUpdate)
                .in_set(UpdateCollisionShapes)
                .after(TransformSystem::TransformPropagate),
        );
    }
}
