use crate::{
    any_component::AnyComponent,
    component::Component,
    component_id::ComponentId,
    controller::Controller,
    object::Object,
    object_id::ObjectId,
    storage::{ControllerStorage, ObjectStorage},
};
use std::num::NonZeroU32;

pub struct Context {
    next_object_id: NonZeroU32,
    next_component_id: NonZeroU32,
    object_storage: ObjectStorage,
    controller_storage: ControllerStorage,
}

impl Context {
    pub fn new() -> Self {
        Self {
            next_object_id: NonZeroU32::MIN,
            next_component_id: NonZeroU32::MIN,
            object_storage: ObjectStorage::new(),
            controller_storage: ControllerStorage::new(),
        }
    }

    pub fn find_object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.object_storage.get(id)
    }

    pub fn find_object_by_id_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.object_storage.get_mut(id)
    }

    pub fn create_object(&mut self) -> ObjectId {
        let object_id = ObjectId::new(self.next_object_id);
        self.next_object_id = self.next_object_id.saturating_add(1);

        let object = Object::new(object_id);
        self.object_storage.add(object);

        object_id
    }

    pub fn create_object_with_components(&mut self, components: Vec<AnyComponent>) -> ObjectId {
        let object_id = ObjectId::new(self.next_object_id);
        self.next_object_id = self.next_object_id.saturating_add(1);

        let object = Object::with_components(object_id, components);
        self.object_storage.add(object);

        object_id
    }

    pub fn remove_object(&mut self, object_id: ObjectId) {
        self.controller_storage.detach_controller(object_id);
        self.object_storage.remove(object_id);
    }

    pub fn add_component<T>(&mut self, object_id: ObjectId, component: T)
    where
        T: Component,
    {
        if let Some(object) = self.object_storage.get_mut(object_id) {
            let component_id = ComponentId::new(self.next_component_id);
            self.next_component_id = self.next_component_id.saturating_add(1);

            let component = AnyComponent::new(component_id, component);
            object.add_component(component);
        }
    }

    pub fn remove_component(&mut self, object_id: ObjectId, component_id: ComponentId) {
        if let Some(object) = self.object_storage.get_mut(object_id) {
            object.remove_component(component_id);
            // TODO: de-allocate the component id
        }
    }

    pub fn attach_controller<T>(&mut self, object_id: ObjectId, controller: T)
    where
        T: Controller,
    {
        let controller = Box::new(controller);
        self.controller_storage
            .attach_controller(object_id, controller);
    }

    pub fn detach_controller(&mut self, object_id: ObjectId) {
        self.controller_storage.detach_controller(object_id);
    }

    pub fn flush_queued_actions(&mut self) {}
}
