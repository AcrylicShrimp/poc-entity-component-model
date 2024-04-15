use crate::{
    any_component::AnyComponent, component::Component, component_id::ComponentId,
    object_id::ObjectId,
};

pub struct Object {
    id: ObjectId,
    components: Vec<AnyComponent>,
}

impl Object {
    pub fn new(id: ObjectId) -> Self {
        Self {
            id,
            components: vec![],
        }
    }

    pub fn with_components(id: ObjectId, components: Vec<AnyComponent>) -> Self {
        Self { id, components }
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn components(&self) -> &[AnyComponent] {
        &self.components
    }

    pub fn find_component_by_id(&self, component_id: ComponentId) -> Option<&AnyComponent> {
        self.components.iter().find(|c| c.id() == component_id)
    }

    pub fn find_component_by_id_mut(
        &mut self,
        component_id: ComponentId,
    ) -> Option<&mut AnyComponent> {
        self.components.iter_mut().find(|c| c.id() == component_id)
    }

    pub fn find_component_by_type<T>(&self) -> Option<&T>
    where
        T: Component,
    {
        self.components.iter().find_map(|c| c.downcast_ref::<T>())
    }

    pub fn find_component_by_type_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Component,
    {
        self.components
            .iter_mut()
            .find_map(|c| c.downcast_mut::<T>())
    }

    pub fn find_components_by_type<T>(&self) -> impl Iterator<Item = &T>
    where
        T: Component,
    {
        self.components.iter().filter_map(|c| c.downcast_ref::<T>())
    }

    pub fn find_components_by_type_mut<T>(&mut self) -> impl Iterator<Item = &mut T>
    where
        T: Component,
    {
        self.components
            .iter_mut()
            .filter_map(|c| c.downcast_mut::<T>())
    }

    pub fn add_component(&mut self, component: AnyComponent) {
        self.components.push(component);
    }

    pub fn remove_component(&mut self, component_id: ComponentId) {
        if let Some(index) = self.components.iter().position(|c| c.id() == component_id) {
            self.components.swap_remove(index);
        }
    }
}
