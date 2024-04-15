use crate::{controller::Controller, object_id::ObjectId, ContextProxy};
use std::collections::{hash_map::Entry, HashMap, HashSet};

pub struct ControllerStorage {
    controllers: HashMap<ObjectId, Box<dyn Controller>>,
    on_update_hooked_controllers: HashSet<ObjectId>,
    on_late_update_hooked_controllers: HashSet<ObjectId>,
}

impl ControllerStorage {
    pub(crate) fn new() -> Self {
        Self {
            controllers: HashMap::new(),
            on_update_hooked_controllers: HashSet::new(),
            on_late_update_hooked_controllers: HashSet::new(),
        }
    }

    pub(crate) fn find_controller(&mut self, id: ObjectId) -> Option<&mut dyn Controller> {
        self.controllers.get_mut(&id).map(|c| c.as_mut())
    }

    pub(crate) fn attach_controller(
        &mut self,
        id: ObjectId,
        controller: Box<dyn Controller>,
        ctx: &mut ContextProxy,
    ) {
        match self.controllers.entry(id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().on_destroy(id, ctx);
                entry.insert(controller);
                entry.get_mut().on_ready(id, ctx);
            }
            Entry::Vacant(entry) => {
                entry.insert(controller).on_ready(id, ctx);
            }
        }
    }

    pub(crate) fn detach_controller(&mut self, id: ObjectId, ctx: &mut ContextProxy) {
        self.on_update_hooked_controllers.remove(&id);
        self.on_late_update_hooked_controllers.remove(&id);

        if let Some(mut controller) = self.controllers.remove(&id) {
            controller.on_destroy(id, ctx);
        }
    }

    pub(crate) fn listen_on_update(&mut self, id: ObjectId) {
        if self.controllers.contains_key(&id) {
            self.on_update_hooked_controllers.insert(id);
        }
    }

    pub(crate) fn unlisten_on_update(&mut self, id: ObjectId) {
        self.on_update_hooked_controllers.remove(&id);
    }

    pub(crate) fn listen_on_late_update(&mut self, id: ObjectId) {
        if self.controllers.contains_key(&id) {
            self.on_late_update_hooked_controllers.insert(id);
        }
    }

    pub(crate) fn unlisten_on_late_update(&mut self, id: ObjectId) {
        self.on_late_update_hooked_controllers.remove(&id);
    }

    pub(crate) fn invoke_on_update(&mut self, ctx: &mut ContextProxy) {
        for id in &self.on_update_hooked_controllers {
            if let Some(controller) = self.controllers.get_mut(id) {
                controller.on_update(*id, ctx);
            }
        }
    }

    pub(crate) fn invoke_on_late_update(&mut self, ctx: &mut ContextProxy) {
        for id in &self.on_late_update_hooked_controllers {
            if let Some(controller) = self.controllers.get_mut(id) {
                controller.on_late_update(*id, ctx);
            }
        }
    }
}
