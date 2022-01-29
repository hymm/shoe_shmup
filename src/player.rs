use crate::actions::Actions;
use crate::bullet::SpawnBullet;
use crate::GameState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands) {
    let shape = shapes::Polygon {
        points: vec![Vec2::new(5., 0.), Vec2::new(-5., 0.), Vec2::new(0., 15.)],
        closed: true,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::rgb_u8(249, 212, 35))),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
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
    if actions.player_point.is_none() {
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
