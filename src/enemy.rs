use crate::bullet::Bullet;
use crate::loading::AudioAssets;
use crate::physics::UpdateCollisionShapes;
use crate::{GameState, LevelEntity};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;

const ENEMY_LENGTH: f32 = 30.;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy_tag: Enemy,
    fill: Fill,
    #[bundle]
    shape_bundle: ShapeBundle,
    collision_shape: CollisionShape,
    level_entity: LevelEntity,
}

impl EnemyBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            enemy_tag: Enemy,
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(ENEMY_LENGTH, ENEMY_LENGTH),
                    origin: shapes::RectangleOrigin::Center,
                }),
                transform,
                ..default()
            },
            fill: Fill::color(Color::rgb_u8(164, 69, 55)),
            collision_shape: CollisionShape::new_rectangle(ENEMY_LENGTH, ENEMY_LENGTH),
            level_entity: LevelEntity,
        }
    }
}

fn after_deserialize_enemy(
    mut commands: Commands,
    q: Query<(Entity, &Transform), (With<Enemy>, Without<CollisionShape>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    if !q.is_empty() {
        state.set(GameState::Playing);
        for (entity, transform) in q.iter() {
            commands.entity(entity).insert(EnemyBundle::new(*transform));
        }
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
            .add_system(
                check_collisions_with_bullets
                    .in_base_set(CoreSet::PostUpdate)
                    .after(UpdateCollisionShapes),
            )
            .add_system(after_deserialize_enemy.in_set(OnUpdate(GameState::PostLoadLevel)));
    }
}
