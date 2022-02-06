use crate::bullet::Bullet;
use crate::loading::AudioAssets;
use crate::physics::UPDATE_COLLISION_SHAPES;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;

#[derive(Component)]
pub(crate) struct Enemy;

fn spawn_enemy(mut commands: Commands) {
    let length = 30.0;
    let shape = shapes::Rectangle {
        extents: Vec2::new(length, length),
        origin: shapes::RectangleOrigin::Center,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::rgb_u8(164, 69, 55))),
            Transform::from_xyz(-20.0, 200.0, 1.0),
        ))
        .insert(Enemy)
        .insert(CollisionShape::new_rectangle(length, length));
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
        for (enemy_entity, enemy_shape) in enemies.iter() {
            if bullet_shape.is_collided_with(enemy_shape) {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                audio.play(audio_assets.explode.clone());
            }
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_collisions_with_bullets.after(UPDATE_COLLISION_SHAPES),
            );
    }
}
