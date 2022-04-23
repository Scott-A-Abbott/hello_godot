use gdnative::init::{godot_init, InitHandle};

mod ecs;
mod hello_world;

fn init(handle: InitHandle) {
    macro_rules! add_class {
        ($($class:ty),* $(,)?) => {
            $( handle.add_class::<$class>(); )*
        };
    }
    add_class![
        ecs::Ecs,
        hello_world::HelloWorld,
        //add more later
    ];
}
godot_init!(init);
