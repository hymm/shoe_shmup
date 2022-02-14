use crate::actions::Actions;
use crate::bullet::{BulletClip, SpawnBullet};
use crate::enemy::Enemy;
use crate::loading::AudioAssets;
use crate::physics::{FixedOffset, Velocity, UPDATE_COLLISION_SHAPES};
use crate::player_rail::{PlayerRail, RailDirection, RailPosition};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_prototype_lyon::entity::ShapeBundle;
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
                SystemSet::on_exit(GameState::Menu)
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
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Velocity(Vec2::new(0.0, 20.0)));
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

#[derive(Component)]
struct RailGraphic;

#[derive(Bundle)]
struct RailShapeBundle {
    tag: RailGraphic,
    #[bundle]
    shape_bundle: ShapeBundle,
    offset: FixedOffset,
}

fn spawn_rail(mut commands: Commands) {
    let rail_points = vec![Vec2::new(-110.0, 0.0), Vec2::new(110.0, 0.0)];
    let rail_color = Color::rgb_u8(135, 188, 108);
    let mut segments = vec![];
    let mut points = vec![RailShapeBundle {
        tag: RailGraphic,
        shape_bundle: GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 10.,
                center: rail_points[0],
            },
            DrawMode::Fill(FillMode::color(rail_color)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
        offset: FixedOffset(Vec2::new(0., -220.)),
    }];

    for (point1, point2) in rail_points[..rail_points.len() - 1]
        .iter()
        .zip(rail_points[1..].iter())
    {
        segments.push(RailShapeBundle {
            tag: RailGraphic,
            shape_bundle: GeometryBuilder::build_as(
                &shapes::Line(*point1, *point2),
                DrawMode::Stroke(StrokeMode::new(rail_color, 5.0)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            offset: FixedOffset(Vec2::new(0., -220.)),
        });

        points.push(RailShapeBundle {
            tag: RailGraphic,
            shape_bundle: GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: 10.,
                    center: *point2,
                },
                DrawMode::Fill(FillMode::color(rail_color)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            offset: FixedOffset(Vec2::new(0., -220.)),
        });
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
    mut player_query: Query<
        (&mut Transform, &mut RailPosition),
        (With<Player>, Without<RailGraphic>),
    >,
    mut clip: Query<&mut BulletClip>,
    rail: Query<&PlayerRail>,
    rail_graphic: Query<&Transform, With<RailGraphic>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    if player_query.is_empty() {
        return;
    }

    if actions.player_stop {
        let (mut player_transform, _) = player_query.single_mut();
        let rail_transform = rail_graphic.iter().next().unwrap();
        player_transform.translation = Vec3::new(
            player_transform.translation.x,
            rail_transform.translation.y,
            player_transform.translation.z,
        );
        return;
    }

    let speed = 150.;

    let mut clip = clip.single_mut();
    let rail = rail.single();
    let (mut player_transform, mut rail_position) = player_query.single_mut();
    let (new_translation, at_node) = rail_position.next_position(rail, time.delta_seconds(), speed);
    if at_node && !clip.is_full() {
        clip.reload();
        audio.play(audio_assets.reload.clone());
    }
    let rail_transform = rail_graphic.iter().next().unwrap();
    player_transform.translation =
        new_translation.extend(0.0) + rail_transform.translation.y * Vec3::Y;
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
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if actions.player_shoot {
        let t = player_query.single_mut();
        let mut clip = clip.single_mut();
        if clip.try_shoot() {
            spawn_bullet.send(SpawnBullet {
                initial_transform: *t,
            });
            audio.play(asset_server.load("audio/shoot.wav"));
        } else {
            audio.play(asset_server.load("audio/empty_clip.wav"));
        }
    }
}

fn check_player_collisions_with_enemies(
    mut commands: Commands,
    player: Query<(Entity, &CollisionShape), (With<Player>, Without<Enemy>)>,
    enemies: Query<(Entity, &CollisionShape), With<Enemy>>,
    audio_assets: Option<Res<AudioAssets>>,
    audio: Res<Audio>,
) {
    if player.iter().next().is_none() || audio_assets.is_none() {
        return;
    }
    let (player_entity, player_shape) = player.single();
    let player_death_sfx = audio_assets.unwrap().player_death.clone();
    for (_enemy_entity, enemy_shape) in enemies.iter() {
        if player_shape.is_collided_with(enemy_shape) {
            commands.entity(player_entity).despawn();
            audio.play(player_death_sfx.clone());
        }
    }
}
