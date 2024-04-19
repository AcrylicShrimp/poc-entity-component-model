use object_system::{Component, ComponentId, Context, ContextProxy, Controller, ObjectId};
use std::any::{type_name, Any};

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

    let empty_object_id = context.with_proxy(|ctx| {
        // create an empty object to test if the `ctx.find_object_ids_by_component_type` correctly ignores objects that does not satisfy the condition
        let object_id = ctx.create_object();
        object_id
    });

    let (dummy_object_id, dummy_component_id) = context.with_proxy(|ctx| {
        // create an object with a dummy component to test if the `ctx.find_object_ids_by_component_type` correctly returns the object id
        let object_id = ctx.create_object();
        let component_id = ctx.add_component(object_id, DummyComponent).unwrap();
        (object_id, component_id)
    });

    context.with_proxy(|ctx| {
        print_all_object_with_component::<MyComponent>(ctx);
    });

    // performs various tests on the `ctx.find_object_ids_by_component_type` method
    context.with_proxy(|ctx| {
        print_all_object_with_component::<DummyComponent>(ctx);
    });
    context.with_proxy(|ctx| {
        ctx.add_component(empty_object_id, DummyComponent).unwrap();
        ctx.remove_component(dummy_object_id, dummy_component_id);

        print_all_object_with_component::<DummyComponent>(ctx);
    });
    context.with_proxy(|ctx| {
        ctx.remove_object(empty_object_id);
        ctx.remove_object(dummy_object_id);
    });
    context.with_proxy(|ctx| {
        print_all_object_with_component::<DummyComponent>(ctx);
    });

    for _ in 0..10 {
        context.proceed_one_frame();
    }
}

fn print_all_object_with_component<T>(ctx: &mut ContextProxy)
where
    T: Component,
{
    let object_ids = ctx.find_object_ids_by_component_type::<T>();

    println!("[objects have component {}]", type_name::<T>());

    if let Some(object_ids) = object_ids {
        for &object_id in object_ids {
            if let Some(object) = ctx.find_object_by_id(object_id) {
                println!("- {:?}", object.id());
            }
        }
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

struct DummyComponent;

impl Component for DummyComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
