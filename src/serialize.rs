use bevy::ecs::schedule::ShouldRun;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;
use bevy::scene::DynamicEntity;
use bevy::reflect::TypeRegistry;

use crate::enemy::Enemy;


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

fn save_scene(world: &mut World) {
  let mut state = SystemState::<SceneParam>::new(world);
  let scene_params = state.get_mut(world);
  let enemies = scene_params.enemies.iter().collect();

  let type_registry = world.get_resource::<TypeRegistry>().unwrap();
  let scene = scene_from_entities(world, type_registry, enemies);
  info!("{}", scene.serialize_ron(type_registry).unwrap());
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
        for entity in archetype.entities().iter().filter(|e| entities.contains(e)) {
            scene.entities.push(DynamicEntity {
                entity: entity.id(),
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
                for (i, entity) in archetype.entities().iter().filter(|e| entities.contains(e)).enumerate() {
                    if let Some(component) = reflect_component.reflect_component(world, *entity) {
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

pub struct SerializePlugin;
impl Plugin for SerializePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                save_scene
                    .exclusive_system()
                    .with_run_criteria(has_save_event),
            )
            .add_event::<SaveSceneEvent>();
    }
}