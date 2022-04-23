use gdnative::prelude::*;

/// This is the most basic way to interact with godot from rust. Just attack a NativeScript to a Node
/// in the scene tree, and make sure it's class name matches the struct, and the library matches the
/// gdnlib file referencing the compiled rust binary (.dll, .so, or what have you)
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
    pub fn new(_owner: TRef<Node>) -> Self {
        Self
    }

    #[export]
    pub fn _ready(&self, _owner: TRef<Node>) {
        godot_print!("Hello, World!");
    }
}
