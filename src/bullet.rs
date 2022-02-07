use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    physics::{FixedOffset, Velocity},
    GameState,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
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

    pub fn is_full(&mut self) -> bool {
        self.bullets >= self.max_size
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

fn despawn_bullet(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    camera: Query<&Transform, (With<Camera>, With<Velocity>)>,
) {
    let camera_transform = camera.single();
    for (e, t) in bullets.iter() {
        let t = t.translation - camera_transform.translation.y * Vec3::Y;
        if t.x > SCREEN_WIDTH / 2.0
            || t.x < -SCREEN_WIDTH / 2.0
            || t.y > SCREEN_HEIGHT / 2.0
            || t.y < -SCREEN_HEIGHT / 2.0
        {
            commands.entity(e).despawn();
        }
    }
}

#[derive(Component)]
struct BulletClipGraphic;

#[derive(Bundle)]
struct BulletClipGraphicBundle {
    tag: BulletClipGraphic,
    #[bundle]
    shape_bundle: ShapeBundle,
    offset: FixedOffset,
}

fn get_bullet_clip_bundles(num_bullets: usize) -> Vec<BulletClipGraphicBundle> {
    let start_point = Vec2::new(-100., -230.);
    (0..num_bullets)
        .map(|i| {
            let point = Vec2::new(start_point.x, start_point.y) + i as f32 * Vec2::new(3., 0.);
            return BulletClipGraphicBundle {
                tag: BulletClipGraphic,
                shape_bundle: GeometryBuilder::build_as(
                    &shapes::Line(point, point + Vec2::new(0.0, -8.0)),
                    DrawMode::Stroke(StrokeMode::new(Color::rgb_u8(0, 0, 0), 2.)),
                    Transform::default(),
                ),
                offset: FixedOffset(Vec2::new(0., 0.)),
            };
        })
        .collect()
}

// show the number of bullets on screen
fn spawn_bullet_clip(mut commands: Commands) {
    const MAX_BULLETS: usize = 5;

    commands.spawn().insert(BulletClip {
        max_size: MAX_BULLETS,
        bullets: MAX_BULLETS,
    });

    commands.spawn_batch(get_bullet_clip_bundles(MAX_BULLETS));
}

fn update_bullet_clip(
    mut commands: Commands,
    clip: Query<&BulletClip, Changed<BulletClip>>,
    graphics: Query<Entity, With<BulletClipGraphic>>,
) {
    if clip.is_empty() {
        return;
    }
    for entity in graphics.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn_batch(get_bullet_clip_bundles(clip.single().bullets));
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBullet>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_bullet_clip))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_bullet)
                    .with_system(despawn_bullet)
                    .with_system(update_bullet_clip),
            );
    }
}
