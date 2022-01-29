use crate::physics::Velocity;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const BULLET_SPEED: f32 = 500.0;

#[derive(Component)]
struct Bullet;

pub struct SpawnBullet {
    pub initial_transform: Transform,
}

fn spawn_bullet(mut commands: Commands, mut spawn_event: EventReader<SpawnBullet>) {
    for ev in spawn_event.iter() {
        let shape = shapes::Circle {
            radius: 4.0,
            ..Default::default()
        };
        // calculate velocity vector based on rotation of character
        let (axis, angle) = ev.initial_transform.rotation.to_axis_angle();
        let direction = Vec2::new(-axis.z * f32::sin(angle), f32::cos(angle));
        let velocity = Velocity(BULLET_SPEED * direction);
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(Color::rgb_u8(255, 255, 255))),
                ev.initial_transform,
            ))
            .insert(Bullet)
            .insert(velocity);
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBullet>().add_system(spawn_bullet);
    }
}
