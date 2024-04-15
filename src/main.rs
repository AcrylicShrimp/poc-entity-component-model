use object_system::{Component, ComponentId, Context, ContextProxy, Controller, ObjectId};
use std::any::Any;

fn main() {
    let mut context = Context::new();

    context.with_proxy(|ctx| {
        let object_id = ctx.create_object();
        let component_1 = ctx
            .add_component(object_id, MyComponent { data: 0 })
            .unwrap();
        let component_2 = ctx
            .add_component(object_id, MyComponent { data: 0 })
            .unwrap();
        let controller = MyController::new(1, component_1, component_2);
        ctx.attach_controller(object_id, controller);
    });

    context.with_proxy(|ctx| {
        let object_id = ctx.create_object();
        let component_1 = ctx
            .add_component(object_id, MyComponent { data: 0 })
            .unwrap();
        let component_2 = ctx
            .add_component(object_id, MyComponent { data: 0 })
            .unwrap();
        let controller = MyController::new(2, component_1, component_2);
        ctx.attach_controller(object_id, controller);
    });

    for _ in 0.. {
        context.proceed_one_frame();
    }
}

struct MyComponent {
    pub data: usize,
}

impl Component for MyComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct MyController {
    id: usize,
    component_1: ComponentId,
    component_2: ComponentId,
}

impl MyController {
    pub fn new(id: usize, component_1: ComponentId, component_2: ComponentId) -> Self {
        Self {
            id,
            component_1,
            component_2,
        }
    }
}

impl Controller for MyController {
    fn on_ready(&mut self, object_id: ObjectId, ctx: &mut ContextProxy) {
        println!("[{:?}] Ready", object_id);
        ctx.listen_on_update(object_id);
        ctx.listen_on_late_update(object_id);

        if self.id == 1 {
            ctx.listen_event("clear-1", object_id);
        } else if self.id == 2 {
            ctx.listen_event("clear-2", object_id);
        }
    }

    fn on_destroy(&mut self, object_id: ObjectId, _ctx: &mut ContextProxy) {
        println!("[{:?}] Destroyed", object_id);
    }

    fn on_update(&mut self, object_id: ObjectId, ctx: &mut ContextProxy) {
        println!("[{:?}] Update", object_id);
        let object = ctx.find_object_by_id_mut(object_id).unwrap();
        let component = object
            .find_component_by_id_mut::<MyComponent>(self.component_1)
            .unwrap();
        component.data += 1;
        println!("[{:?}] Component data: {}", object_id, component.data);

        if self.id == 1 && component.data == 3 {
            ctx.emit_event("clear-2", ());
        }
    }

    fn on_late_update(&mut self, object_id: ObjectId, ctx: &mut ContextProxy) {
        println!("[{:?}] Late update", object_id);
        let object = ctx.find_object_by_id_mut(object_id).unwrap();
        let component = object
            .find_component_by_id_mut::<MyComponent>(self.component_2)
            .unwrap();
        component.data += 1;
        println!("[{:?}] Component data: {}", object_id, component.data);

        if self.id == 2 && component.data == 4 {
            ctx.emit_event("clear-1", ());
        }
    }

    fn on_event(
        &mut self,
        event: &str,
        _param: &dyn Any,
        object_id: ObjectId,
        ctx: &mut ContextProxy,
    ) {
        match event {
            "clear-1" => {
                println!("[{:?}] Clear 1", object_id);
                let object = ctx.find_object_by_id_mut(object_id).unwrap();
                let component_1 = object
                    .find_component_by_id_mut::<MyComponent>(self.component_1)
                    .unwrap();
                component_1.data = 0;
                let component_2 = object
                    .find_component_by_id_mut::<MyComponent>(self.component_2)
                    .unwrap();
                component_2.data = 0;
            }
            "clear-2" => {
                println!("[{:?}] Clear 2", object_id);
                let object = ctx.find_object_by_id_mut(object_id).unwrap();
                let component = object
                    .find_component_by_id_mut::<MyComponent>(self.component_2)
                    .unwrap();
                component.data = 0;
                let component_1 = object
                    .find_component_by_id_mut::<MyComponent>(self.component_1)
                    .unwrap();
                component_1.data = 0;
            }
            _ => {}
        }
    }
}
