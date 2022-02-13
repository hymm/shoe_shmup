use crate::bullet::Bullet;
use crate::loading::AudioAssets;
use crate::physics::UPDATE_COLLISION_SHAPES;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;

const ENEMY_LENGTH: f32 = 30.;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy_tag: Enemy,
    #[bundle]
    shape_bundle: ShapeBundle,
    collision_shape: CollisionShape,
}

impl EnemyBundle {
    fn new(transform: Transform) -> Self {
        Self {
            enemy_tag: Enemy,
            shape_bundle: GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::new(ENEMY_LENGTH, ENEMY_LENGTH),
                    origin: shapes::RectangleOrigin::Center,
                },
                DrawMode::Fill(FillMode::color(Color::rgb_u8(164, 69, 55))),
                transform,
            ),
            collision_shape: CollisionShape::new_rectangle(ENEMY_LENGTH, ENEMY_LENGTH),
        }
    }
}

fn spawn_enemy(mut commands: Commands) {
    let mut bundles = vec![];
    for x in 0..7 {
        for y in 0..5 {
            bundles.push(EnemyBundle::new(Transform::from_xyz(
                -105.0 + x as f32 * (ENEMY_LENGTH + 5.0),
                220.0 - y as f32 * (ENEMY_LENGTH + 5.0),
                1.0,
            )))
        }
    }
    commands.spawn_batch(bundles);
}

fn after_deserialize_enemy(
    mut commands: Commands,
    q: Query<(Entity, &Transform), (With<Enemy>, Without<CollisionShape>)>,
) {
    for (entity, transform) in q.iter() {
        commands
            .entity(entity)
            .insert_bundle(EnemyBundle::new(*transform));
    }
}

fn check_collisions_with_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &CollisionShape), (With<Bullet>, Without<Enemy>)>,
    enemies: Query<(Entity, &CollisionShape), With<Enemy>>,
    audio_assets: Option<Res<AudioAssets>>,
    audio: Res<Audio>,
) {
    if audio_assets.is_none() {
        return;
    }
    let audio_assets = audio_assets.unwrap();
    for (bullet_entity, bullet_shape) in bullets.iter() {
        let mut bullet_collided = false;
        for (enemy_entity, enemy_shape) in enemies.iter() {
            if bullet_shape.is_collided_with(enemy_shape) {
                commands.entity(enemy_entity).despawn();
                audio.play(audio_assets.explode.clone());
                bullet_collided = true;
            }
        }
        if bullet_collided {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_collisions_with_bullets.after(UPDATE_COLLISION_SHAPES),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PostLoadLevel).with_system(after_deserialize_enemy),
            );
    }
}
