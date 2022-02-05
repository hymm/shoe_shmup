use crate::actions::Actions;
use crate::bullet::{BulletClip, SpawnBullet};
use crate::enemy::Enemy;
use crate::physics::UPDATE_COLLISION_SHAPES;
use crate::player_rail::{PlayerRail, RailDirection, RailPosition};
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
                    .with_system(spawn_camera)
                    .with_system(spawn_rail),
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
        points: vec![Vec2::new(10., 0.), Vec2::new(-10., 0.), Vec2::new(0., 30.)],
        closed: true,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::rgb_u8(199, 167, 37))),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .insert(Player)
        .insert(CollisionShape::new_rectangle(8.0, 12.0))
        .insert(RailPosition {
            index: 0,
            position: 0.0,
            direction: RailDirection::Positive,
        });
}

fn spawn_rail(mut commands: Commands) {
    let rail_points = vec![Vec2::new(-110.0, -220.0), Vec2::new(110.0, -220.0)];
    let rail_color = Color::rgb_u8(135, 188, 108);
    let mut segments = vec![];
    let mut points = vec![];

    points.push(GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 10.,
            center: rail_points[0],
        },
        DrawMode::Fill(FillMode::color(rail_color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    for (point1, point2) in rail_points[..rail_points.len() - 1]
        .iter()
        .zip(rail_points[1..].iter())
    {
        segments.push(GeometryBuilder::build_as(
            &shapes::Line(*point1, *point2),
            DrawMode::Stroke(StrokeMode::new(rail_color, 5.0)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        points.push(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 10.,
                center: *point2,
            },
            DrawMode::Fill(FillMode::color(rail_color)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    commands.spawn_batch(segments);
    commands.spawn_batch(points);
    commands.spawn().insert(PlayerRail {
        rail: rail_points,
        closed: false,
    });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut RailPosition), With<Player>>,
    mut clip: Query<&mut BulletClip>,
    rail: Query<&PlayerRail>,
) {
    if player_query.is_empty() || actions.player_stop {
        return;
    }
    let speed = 150.;

    let mut clip = clip.single_mut();
    let (mut player_transform, mut rail_position) = player_query.single_mut();
    let (t, at_node) = rail_position.next_position(&rail.single(), time.delta_seconds(), speed);
    if at_node {
        clip.reload();
    }
    player_transform.translation = t.extend(0.0);
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
    mut player_query: Query<&Transform, With<Player>>,
    mut clip: Query<&mut BulletClip>,
    mut spawn_bullet: EventWriter<SpawnBullet>,
) {
    if actions.player_shoot {
        let t = player_query.single_mut();
        let mut clip = clip.single_mut();
        if clip.try_shoot() {
            spawn_bullet.send(SpawnBullet {
                initial_transform: t.clone(),
            })
        }
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
