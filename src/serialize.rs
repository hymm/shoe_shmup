use async_compat::Compat;
use bevy::ecs::schedule::ShouldRun;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;
use bevy::scene::DynamicEntity;
use bevy::tasks::{IoTaskPool, Task};
use futures_lite::future;
use tokio::fs;

use crate::enemy::Enemy;
use crate::GameState;

pub struct SaveSceneEvent;

// TODO: move to serialize file
fn has_save_event(mut e: EventReader<SaveSceneEvent>) -> ShouldRun {
    let mut result = ShouldRun::No;
    // iterate over all events to drain
    for _ in e.iter() {
        result = ShouldRun::Yes;
    }
    result
}

#[derive(SystemParam)]
struct SceneParam<'w, 's> {
    enemies: Query<'w, 's, Entity, With<Enemy>>,
}

#[derive(Component)]
struct SaveTask(pub Task<()>);

fn save_scene(world: &mut World) {
    let mut state = SystemState::<SceneParam>::new(world);
    let scene_params = state.get_mut(world);
    let enemies = scene_params.enemies.iter().collect();

    let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let scene = scene_from_entities(world, type_registry, enemies);
    let scene = scene.serialize_ron(type_registry).unwrap();
    let task = Compat::new(async {
        let result = fs::write("assets/levels/temp.ron", scene).await;
        if let Err(error) = result {
            dbg!(error);
        } else {
            dbg!("saved");
        }
    });
    dbg!("try save");
    let task = IoTaskPool::get().spawn(task);

    world.spawn(SaveTask(task));
}

fn handle_save_task(mut commands: Commands, mut save_task: Query<(Entity, &mut SaveTask)>) {
    if let Ok((entity, mut task)) = save_task.get_single_mut() {
        future::block_on(future::poll_once(&mut task.0));
        // TODO: this is wrong, only should despawn when task is done.
        commands.entity(entity).despawn();
    }
}

pub fn scene_from_entities(
    world: &World,
    type_registry: &TypeRegistryArc,
    entities: Vec<Entity>,
) -> DynamicScene {
    let mut scene = DynamicScene::default();
    let type_registry = type_registry.read();

    for archetype in world.archetypes().iter() {
        let entities_offset = scene.entities.len();

        // Create a new dynamic entity for each entity of the given archetype
        // and insert it into the dynamic scene.
        for entity in archetype
            .entities()
            .iter()
            .filter(|e| entities.contains(&e.entity()))
        {
            scene.entities.push(DynamicEntity {
                entity: entity.entity().index(),
                components: Vec::new(),
            });
        }

        // Add each reflection-powered component to the entity it belongs to.
        for component_id in archetype.components() {
            let reflect_component = world
                .components()
                .get_info(component_id)
                .and_then(|info| type_registry.get(info.type_id().unwrap()))
                .and_then(|registration| registration.data::<ReflectComponent>());
            if let Some(reflect_component) = reflect_component {
                for (i, entity) in archetype
                    .entities()
                    .iter()
                    .filter(|e| entities.contains(&e.entity()))
                    .enumerate()
                {
                    if let Some(component) = reflect_component.reflect(world, entity.entity()) {
                        scene.entities[entities_offset + i]
                            .components
                            .push(component.clone_value());
                    }
                }
            }
        }
    }

    scene
}

fn load_scene(
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut state: ResMut<State<GameState>>,
) {
    let scene_handle = asset_server.load("levels/level1.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    state.overwrite_replace(GameState::PostLoadLevel).unwrap();
}

pub struct SerializePlugin;
impl Plugin for SerializePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(save_scene.with_run_criteria(has_save_event))
            .add_system_set(SystemSet::on_enter(GameState::LoadLevel).with_system(load_scene))
            .add_system(handle_save_task)
            .add_event::<SaveSceneEvent>();
    }
}
