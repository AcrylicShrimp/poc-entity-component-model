use crate::{object_id::ObjectId, ContextProxy};
use std::any::Any;

pub trait Controller: Any {
    fn on_ready(&mut self, _object_id: ObjectId, _ctx: &mut ContextProxy) {}
    fn on_destroy(&mut self, _object_id: ObjectId, _ctx: &mut ContextProxy) {}
    fn on_update(&mut self, _object_id: ObjectId, _ctx: &mut ContextProxy) {}
    fn on_late_update(&mut self, _object_id: ObjectId, _ctx: &mut ContextProxy) {}
    fn on_event(
        &mut self,
        _event: &str,
        _param: &dyn Any,
        _object_id: ObjectId,
        _ctx: &mut ContextProxy,
    ) {
    }
}
