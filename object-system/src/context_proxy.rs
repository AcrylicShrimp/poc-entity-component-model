use crate::{AnyComponent, Component, ComponentId, Controller, Object, ObjectId, ObjectStorage};
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    num::NonZeroU32,
};

pub(crate) enum ContextActionItem {
    RemoveObject {
        object_id: ObjectId,
    },
    AttachController {
        object_id: ObjectId,
        controller: Box<dyn Controller>,
    },
    DetachController {
        object_id: ObjectId,
    },
    ListenOnUpdate {
        object_id: ObjectId,
    },
    UnlistenOnUpdate {
        object_id: ObjectId,
    },
    ListenOnLateUpdate {
        object_id: ObjectId,
    },
    UnlistenOnLateUpdate {
        object_id: ObjectId,
    },
    ListenEvent {
        event: String,
        object_id: ObjectId,
    },
    UnlistenEvent {
        event: String,
        object_id: ObjectId,
    },
    UnlistenEventAll {
        object_id: ObjectId,
    },
    EmitEvent {
        event: String,
        param: Box<dyn Any>,
    },
}

pub(crate) struct ContextResult {
    pub next_object_id: NonZeroU32,
    pub next_component_id: NonZeroU32,
    pub action_queue: Vec<ContextActionItem>,
}

pub struct ContextProxy<'ctx> {
    next_object_id: NonZeroU32,
    next_component_id: NonZeroU32,
    object_storage: &'ctx mut ObjectStorage,
    action_queue: Vec<ContextActionItem>,
}

impl<'ctx> ContextProxy<'ctx> {
    pub(crate) fn new(
        next_object_id: NonZeroU32,
        next_component_id: NonZeroU32,
        object_storage: &'ctx mut ObjectStorage,
    ) -> Self {
        Self {
            next_object_id,
            next_component_id,
            object_storage,
            action_queue: Vec::new(),
        }
    }

    pub(crate) fn into_result(self) -> ContextResult {
        ContextResult {
            next_object_id: self.next_object_id,
            next_component_id: self.next_component_id,
            action_queue: self.action_queue,
        }
    }

    pub fn find_object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.object_storage.get(id)
    }

    pub fn find_object_by_id_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.object_storage.get_mut(id)
    }

    pub fn find_object_ids_by_component_type<T>(&self) -> Option<&HashSet<ObjectId>>
    where
        T: Component,
    {
        self.object_storage.object_ids_with_component::<T>()
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
        self.action_queue
            .push(ContextActionItem::RemoveObject { object_id });
    }

    pub fn add_component<T>(&mut self, object_id: ObjectId, component: T) -> Option<ComponentId>
    where
        T: Component,
    {
        match self.object_storage.get_mut(object_id) {
            Some(object) => {
                let component_id = ComponentId::new(self.next_component_id);
                self.next_component_id = self.next_component_id.saturating_add(1);

                let component = AnyComponent::new(component_id, component);
                object.add_component(component);

                self.object_storage
                    .register_component(object_id, TypeId::of::<T>());

                Some(component_id)
            }
            None => None,
        }
    }

    pub fn remove_component(&mut self, object_id: ObjectId, component_id: ComponentId) {
        if let Some(object) = self.object_storage.get_mut(object_id) {
            if let Some(component) = object.remove_component(component_id) {
                self.object_storage
                    .unregister_component(object_id, component.type_id());

                // TODO: de-allocate the component id
            }
        }
    }

    pub fn attach_controller<T>(&mut self, object_id: ObjectId, controller: T)
    where
        T: Controller,
    {
        self.action_queue.push(ContextActionItem::AttachController {
            object_id,
            controller: Box::new(controller),
        });
    }

    pub fn detach_controller(&mut self, object_id: ObjectId) {
        self.action_queue
            .push(ContextActionItem::DetachController { object_id });
    }

    pub fn listen_on_update(&mut self, object_id: ObjectId) {
        self.action_queue
            .push(ContextActionItem::ListenOnUpdate { object_id });
    }

    pub fn unlisten_on_update(&mut self, object_id: ObjectId) {
        self.action_queue
            .push(ContextActionItem::UnlistenOnUpdate { object_id });
    }

    pub fn listen_on_late_update(&mut self, object_id: ObjectId) {
        self.action_queue
            .push(ContextActionItem::ListenOnLateUpdate { object_id });
    }

    pub fn unlisten_on_late_update(&mut self, object_id: ObjectId) {
        self.action_queue
            .push(ContextActionItem::UnlistenOnLateUpdate { object_id });
    }

    pub fn listen_event(&mut self, event: impl Into<String>, object_id: ObjectId) {
        self.action_queue.push(ContextActionItem::ListenEvent {
            event: event.into(),
            object_id,
        });
    }

    pub fn unlisten_event(&mut self, event: impl Into<String>, object_id: ObjectId) {
        self.action_queue.push(ContextActionItem::UnlistenEvent {
            event: event.into(),
            object_id,
        });
    }

    pub fn unlisten_event_all(&mut self, object_id: ObjectId) {
        self.action_queue
            .push(ContextActionItem::UnlistenEventAll { object_id });
    }

    pub fn emit_event(&mut self, event: impl Into<String>, param: impl Any) {
        self.action_queue.push(ContextActionItem::EmitEvent {
            event: event.into(),
            param: Box::new(param),
        });
    }
}
