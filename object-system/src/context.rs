use crate::{
    storage::{ControllerStorage, ObjectStorage},
    ContextActionItem, ContextProxy, ContextResult, EventReceiverStorage,
};
use std::num::NonZeroU32;

pub struct Context {
    next_object_id: NonZeroU32,
    next_component_id: NonZeroU32,
    object_storage: ObjectStorage,
    event_receiver_storage: EventReceiverStorage,
    controller_storage: ControllerStorage,
}

impl Context {
    pub fn new() -> Self {
        Self {
            next_object_id: NonZeroU32::MIN,
            next_component_id: NonZeroU32::MIN,
            object_storage: ObjectStorage::new(),
            event_receiver_storage: EventReceiverStorage::new(),
            controller_storage: ControllerStorage::new(),
        }
    }

    pub fn with_proxy<R>(&mut self, f: impl FnOnce(&mut ContextProxy) -> R) -> R {
        let mut ctx = ContextProxy::new(
            self.next_object_id,
            self.next_component_id,
            &mut self.object_storage,
        );
        let result = f(&mut ctx);
        let ctx_result = ctx.into_result();
        self.handle_context_result(ctx_result);
        result
    }

    pub fn proceed_one_frame(&mut self) {
        let mut ctx = ContextProxy::new(
            self.next_object_id,
            self.next_component_id,
            &mut self.object_storage,
        );
        self.controller_storage.invoke_on_update(&mut ctx);

        let result = ctx.into_result();
        self.handle_context_result(result);

        let mut ctx = ContextProxy::new(
            self.next_object_id,
            self.next_component_id,
            &mut self.object_storage,
        );
        self.controller_storage.invoke_on_late_update(&mut ctx);

        let result = ctx.into_result();
        self.handle_context_result(result);
    }

    fn handle_context_result(&mut self, mut result: ContextResult) {
        let mut removed_objects = vec![];

        while !result.action_queue.is_empty() {
            let mut ctx = ContextProxy::new(
                result.next_object_id,
                result.next_component_id,
                &mut self.object_storage,
            );

            for action in result.action_queue {
                match action {
                    ContextActionItem::RemoveObject { object_id } => {
                        self.event_receiver_storage.unlisten_all(object_id);
                        self.controller_storage
                            .detach_controller(object_id, &mut ctx);
                        removed_objects.push(object_id);
                    }
                    ContextActionItem::AttachController {
                        object_id,
                        controller,
                    } => {
                        self.controller_storage
                            .attach_controller(object_id, controller, &mut ctx);
                    }
                    ContextActionItem::DetachController { object_id } => {
                        self.event_receiver_storage.unlisten_all(object_id);
                        self.controller_storage
                            .detach_controller(object_id, &mut ctx);
                    }
                    ContextActionItem::ListenOnUpdate { object_id } => {
                        self.controller_storage.listen_on_update(object_id);
                    }
                    ContextActionItem::UnlistenOnUpdate { object_id } => {
                        self.controller_storage.unlisten_on_update(object_id);
                    }
                    ContextActionItem::ListenOnLateUpdate { object_id } => {
                        self.controller_storage.listen_on_late_update(object_id);
                    }
                    ContextActionItem::UnlistenOnLateUpdate { object_id } => {
                        self.controller_storage.unlisten_on_late_update(object_id);
                    }
                    ContextActionItem::ListenEvent { event, object_id } => {
                        self.event_receiver_storage.listen(event, object_id);
                    }
                    ContextActionItem::UnlistenEvent { event, object_id } => {
                        self.event_receiver_storage.unlisten(event, object_id);
                    }
                    ContextActionItem::UnlistenEventAll { object_id } => {
                        self.event_receiver_storage.unlisten_all(object_id);
                    }
                    ContextActionItem::EmitEvent { event, param } => {
                        self.event_receiver_storage.emit(
                            &event,
                            &param,
                            &mut ctx,
                            &mut self.controller_storage,
                        );
                    }
                }
            }

            result = ctx.into_result();
        }

        for object_id in removed_objects {
            self.object_storage.remove(object_id);
        }

        self.next_object_id = result.next_object_id;
        self.next_component_id = result.next_component_id;
    }
}
