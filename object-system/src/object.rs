use crate::{
    any_component::AnyComponent, component::Component, component_id::ComponentId,
    object_id::ObjectId,
};

pub struct Object {
    id: ObjectId,
    components: Vec<AnyComponent>,
}

impl Object {
    pub(crate) fn new(id: ObjectId) -> Self {
        Self {
            id,
            components: vec![],
        }
    }

    pub(crate) fn with_components(id: ObjectId, components: Vec<AnyComponent>) -> Self {
        Self { id, components }
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn components(&self) -> &[AnyComponent] {
        &self.components
    }

    pub fn find_component_by_id<T>(&self, component_id: ComponentId) -> Option<&T>
    where
        T: Component,
    {
        self.components
            .iter()
            .find(|c| c.id() == component_id)
            .and_then(|c| c.downcast_ref::<T>())
    }

    pub fn find_component_by_id_mut<T>(&mut self, component_id: ComponentId) -> Option<&mut T>
    where
        T: Component,
    {
        self.components
            .iter_mut()
            .find(|c| c.id() == component_id)
            .and_then(|c| c.downcast_mut::<T>())
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

    pub(crate) fn add_component(&mut self, component: AnyComponent) {
        self.components.push(component);
    }

    pub(crate) fn remove_component(&mut self, component_id: ComponentId) -> Option<AnyComponent> {
        self.components
            .iter()
            .position(|c| c.id() == component_id)
            .map(|index| self.components.swap_remove(index))
    }
}
