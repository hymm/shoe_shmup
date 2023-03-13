use crate::actions::{Actions, ActionsSet};
use crate::bullet::{BulletClip, SpawnBullet};
use crate::enemy::Enemy;
use crate::loading::AudioAssets;
use crate::physics::{FixedOffset, UpdateCollisionShapes};
use crate::player_rail::{PlayerRail, RailDirection, RailPosition};
use crate::{GameState, LevelEntity};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
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
            .add_systems((spawn_player, spawn_rail).in_schedule(OnExit(GameState::Menu)))
            .add_systems(
                (move_player, point_player, player_shoot)
                    .in_set(OnUpdate(GameState::Playing))
                    .after(ActionsSet),
            )
            .add_system(
                check_player_collisions_with_enemies
                    .in_base_set(CoreSet::PostUpdate)
                    .after(UpdateCollisionShapes),
            )
            .add_system(back_to_menu.in_schedule(OnEnter(GameState::PlayerDead)));
    }
}

fn spawn_player(mut commands: Commands) {
    let shape = shapes::Polygon {
        points: vec![Vec2::new(10., 0.), Vec2::new(-10., 0.), Vec2::new(0., 30.)],
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::rgb_u8(199, 167, 37)),
        Player,
        CollisionShape::new_rectangle(8.0, 12.0),
        RailPosition {
            index: 0,
            position: 0.0,
            direction: RailDirection::Positive,
        },
        LevelEntity,
    ));
}

#[derive(Component)]
struct RailGraphic;

#[derive(Bundle)]
struct RailShapeFillBundle {
    tag: RailGraphic,
    fill: Fill,
    #[bundle]
    shape_bundle: ShapeBundle,
    offset: FixedOffset,
    level_entity: LevelEntity,
}

#[derive(Bundle)]
struct RailShapeStrokeBundle {
    tag: RailGraphic,
    stroke: Stroke,
    #[bundle]
    shape_bundle: ShapeBundle,
    offset: FixedOffset,
    level_entity: LevelEntity,
}

fn spawn_rail(mut commands: Commands) {
    let rail_points = vec![Vec2::new(-110.0, 0.0), Vec2::new(110.0, 0.0)];
    let rail_color = Color::rgb_u8(135, 188, 108);
    let mut segments = vec![];
    let mut points = vec![RailShapeFillBundle {
        tag: RailGraphic,

        fill: Fill::color(rail_color),
        shape_bundle: ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 10.,
                center: rail_points[0],
            }),
            ..default()
        },
        offset: FixedOffset(Vec2::new(0., -220.)),
        level_entity: LevelEntity,
    }];

    for (point1, point2) in rail_points[..rail_points.len() - 1]
        .iter()
        .zip(rail_points[1..].iter())
    {
        segments.push((RailShapeStrokeBundle {
            tag: RailGraphic,
            stroke: Stroke::new(rail_color, 5.0),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(*point1, *point2)),
                ..default()
            },
            offset: FixedOffset(Vec2::new(0., -220.)),
            level_entity: LevelEntity,
        },));

        points.push(RailShapeFillBundle {
            tag: RailGraphic,
            fill: Fill::color(rail_color),
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 10.,
                    center: *point2,
                }),
                ..default()
            },
            offset: FixedOffset(Vec2::new(0., -220.)),
            level_entity: LevelEntity,
        });
    }

    commands.spawn_batch(segments);
    commands.spawn_batch(points);
    commands.spawn((
        PlayerRail {
            rail: rail_points,
            closed: false,
        },
        LevelEntity,
    ));
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
    if at_node && !clip.full() {
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
    mut state: ResMut<NextState<GameState>>,
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
            state.set(GameState::PlayerDead);
        }
    }
}

fn back_to_menu(
    mut state: ResMut<NextState<GameState>>,
    q: Query<Entity, With<LevelEntity>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut commands: Commands,
) {
    state.set(GameState::Menu);
    for e in &q {
        commands.entity(e).despawn();
    }

    let mut transform = camera.single_mut();
    transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
}
