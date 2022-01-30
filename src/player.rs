use crate::actions::Actions;
use crate::bullet::SpawnBullet;
use crate::enemy::Enemy;
use crate::physics::UPDATE_COLLISION_SHAPES;
use crate::GameState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player)
                    .with_system(spawn_camera),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .after("actions")
                    .with_system(move_player)
                    .with_system(point_player)
                    .with_system(player_shoot),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .after(UPDATE_COLLISION_SHAPES)
                    .with_system(check_player_collisions_with_enemies),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands) {
    let shape = shapes::Polygon {
        points: vec![Vec2::new(5., 0.), Vec2::new(-8., 0.), Vec2::new(0., 20.)],
        closed: true,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::rgb_u8(199, 167, 37))),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .insert(Player)
        .insert(CollisionShape::new_rectangle(8.0, 12.0));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if player_query.is_empty() || actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}

fn point_player(actions: Res<Actions>, mut player_query: Query<&mut Transform, With<Player>>) {
    if player_query.is_empty() || actions.player_point.is_none() {
        return;
    }
    let mut player_transform = player_query.single_mut();
    let target = actions.player_point.unwrap();

    let forward = Vec2::normalize(target - player_transform.translation.truncate());
    let angle = Vec2::Y.angle_between(forward);
    player_transform.rotation = Quat::from_rotation_z(angle);
}

fn player_shoot(
    actions: Res<Actions>,
    player_query: Query<&Transform, With<Player>>,
    mut spawn_bullet: EventWriter<SpawnBullet>,
) {
    if actions.player_shoot {
        let t = player_query.single();
        spawn_bullet.send(SpawnBullet {
            initial_transform: t.clone(),
        })
    }
}

fn check_player_collisions_with_enemies(
    mut commands: Commands,
    player: Query<(Entity, &CollisionShape), (With<Player>, Without<Enemy>)>,
    enemies: Query<(Entity, &CollisionShape), With<Enemy>>,
) {
    if player.iter().next().is_none() {
        return;
    }
    let (player_entity, player_shape) = player.single();

    for (_enemy_entity, enemy_shape) in enemies.iter() {
        if player_shape.is_collided_with(enemy_shape) {
            commands.entity(player_entity).despawn();
        }
    }
}
