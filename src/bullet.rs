use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    physics::Velocity,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;

const BULLET_SPEED: f32 = 500.0;

#[derive(Component)]
pub(crate) struct Bullet;

pub struct SpawnBullet {
    pub initial_transform: Transform,
}

#[derive(Component)]
pub struct BulletClip {
    pub max_size: usize,
    pub bullets: usize,
}

impl BulletClip {
    pub fn try_shoot(&mut self) -> bool {
        if self.bullets > 0 {
            self.bullets -= 1;
            true
        } else {
            false
        }
    }

    pub fn reload(&mut self) {
        self.bullets = self.max_size;
    }
}

fn spawn_bullet(mut commands: Commands, mut spawn_event: EventReader<SpawnBullet>) {
    let bullet_radius = 4.0;
    for ev in spawn_event.iter() {
        let shape = shapes::Circle {
            radius: bullet_radius,
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
            .insert(velocity)
            .insert(CollisionShape::new_circle(bullet_radius));
    }
}

fn despawn_bullet(mut commands: Commands, bullets: Query<(Entity, &Transform), With<Bullet>>) {
    for (e, t) in bullets.iter() {
        if t.translation.x > SCREEN_WIDTH / 2.0
            || t.translation.x < -SCREEN_WIDTH / 2.0
            || t.translation.y > SCREEN_HEIGHT / 2.0
            || t.translation.y < -SCREEN_HEIGHT / 2.0
        {
            commands.entity(e).despawn();
        }
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBullet>()
            .add_system(spawn_bullet)
            .add_system(despawn_bullet);
    }
}
