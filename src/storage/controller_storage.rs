use crate::{controller::Controller, object_id::ObjectId};
use std::collections::{HashMap, HashSet};

pub struct ControllerStorage {
    controllers: HashMap<ObjectId, Box<dyn Controller>>,
    on_update_hooked_controllers: HashSet<ObjectId>,
    on_late_update_hooked_controllers: HashSet<ObjectId>,
}

impl ControllerStorage {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
            on_update_hooked_controllers: HashSet::new(),
            on_late_update_hooked_controllers: HashSet::new(),
        }
    }

    pub fn attach_controller(&mut self, id: ObjectId, controller: Box<dyn Controller>) {
        self.controllers
            .entry(id)
            .or_insert(controller)
            .on_ready(id);
    }

    pub fn detach_controller(&mut self, id: ObjectId) {
        self.on_update_hooked_controllers.remove(&id);
        self.on_late_update_hooked_controllers.remove(&id);

        if let Some(mut controller) = self.controllers.remove(&id) {
            controller.on_destroy(id);
        }
    }

    pub fn invoke_on_update(&mut self) {
        for id in &self.on_update_hooked_controllers {
            if let Some(controller) = self.controllers.get_mut(id) {
                controller.on_update(*id);
            }
        }
    }

    pub fn invoke_on_late_update(&mut self) {
        for id in &self.on_late_update_hooked_controllers {
            if let Some(controller) = self.controllers.get_mut(id) {
                controller.on_late_update(*id);
            }
        }
    }
}
