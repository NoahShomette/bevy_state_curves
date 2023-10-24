use bevy::{
    ecs::system::Command,
    prelude::{AppTypeRegistry, Entity, ReflectComponent, World},
    reflect::Reflect,
};

/// A [`Command`] that adds the boxed reflect component to an entity using the data in
/// [`AppTypeRegistry`].
///
/// See [`ReflectCommandExt::insert_reflect`] for details.
pub struct InsertReflect {
    /// The entity on which the component will be inserted.
    pub entity: Entity,
    /// The reflect [Component](crate::component::Component) that will be added to the entity.
    pub component: Box<dyn Reflect>,
}

impl Command for InsertReflect {
    fn apply(self, world: &mut World) {
        let registry = world.get_resource::<AppTypeRegistry>().unwrap().clone();
        let type_registry = registry.read();
        let type_info = self.component.type_name();
        let Some(mut entity) = world.get_entity_mut(self.entity) else {
            panic!("error[B0003]: Could not insert a reflected component (of type {}) for entity {:?} because it doesn't exist in this World.", self.component.type_name(),self.entity);
        };
        let Some(type_registration) = type_registry.get_with_name(type_info) else {
            panic!("Could not get type registration (for component type {}) because it doesn't exist in the TypeRegistry.", self.component.type_name());
        };
        let Some(reflect_component) = type_registration.data::<ReflectComponent>() else {
            panic!("Could not get ReflectComponent data (for component type {}) because it doesn't exist in this TypeRegistration.", self.component.type_name());
        };
        reflect_component.insert(&mut entity, &*self.component);
    }
}
