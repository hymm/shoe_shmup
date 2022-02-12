use bevy::prelude::{DynamicScene, Entity, ReflectComponent, World};
use bevy::reflect::TypeRegistryArc;
use bevy::scene::DynamicEntity;

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
                for (i, entity) in archetype.entities().iter().enumerate() {
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
