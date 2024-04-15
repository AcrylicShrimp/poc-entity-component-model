use crate::{object::Object, object_id::ObjectId};
use std::collections::HashMap;

pub struct ObjectStorage {
    objects: HashMap<ObjectId, Object>,
}

impl ObjectStorage {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.entry(object.id()).or_insert(object);
    }

    pub fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
        // TODO: de-allocate the object id
    }

    pub fn get(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }
}
