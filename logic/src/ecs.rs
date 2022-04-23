use bevy::{ecs::system::CommandQueue, prelude::*};
use gdnative::prelude::{self as gd, *};
use iyes_loopless::prelude::*;

/*
A simplified version could have looked like this:

#[derive(NativeClass, Deref, DerefMut)]
#[inherit(gd::Node)]
pub struct Ecs(App);

#[methods]
impl Ecs {
    pub fn new(_owner: TRef<Node>) -> Self {
        let mut app = App::empty();

        app.add_default_stages()
            .add_system_to_stage(CoreStage::Last, World::clear_trackers.exclusive_system())
            .add_startup_system(hello_bevy);

        Self(app)
    }

    #[export]
    pub fn _ready(&self, _owner: TRef<Node>) {
        self.update();
    }
    
    #[export]
    pub fn _process(&self, _owner: TRef<Node>, _delta: f64) {
        self.update();
    }
}
*/

#[derive(NativeClass, Deref, DerefMut)]
#[inherit(gd::Node)]
pub struct Ecs(App);

impl Default for Ecs {
    /// This replaces bevy's CorePlugin, because the CorePlugin requires reflecting types.
    fn default() -> Self {
        let mut app = App::empty();
        DefaultTaskPoolOptions::default().create_default_pools(&mut app.world);

        app.add_default_stages()
            .insert_resource(Time::default())
            .add_system_to_stage(
                CoreStage::First,
                (|mut time: ResMut<Time>| time.update()).exclusive_system(),
            )
            .add_system_to_stage(CoreStage::Last, World::clear_trackers.exclusive_system());

        Self(app)
    }
}

impl Ecs {
    /// Helper for running commands outside the ecs world
    pub fn run_commands(&mut self, command_fn: impl FnOnce(Commands)) {
        let mut queue = CommandQueue::default();
        let world = &mut self.world;
        let commands = Commands::new(&mut queue, world);

        command_fn(commands);
        queue.apply(world);
    }
}

#[methods]
impl Ecs {
    pub fn new(owner: TRef<gd::Node>) -> Self {
        let mut ecs = Self::default();

        ecs.add_loopless_state(ProcessState::Idle)
            .insert_resource(EcsNode(owner.claim()))
            .add_startup_system(hello_bevy)
            .add_system(exit_on_escape.run_in_state(ProcessState::Idle));

        ecs
    }

    #[export]
    pub fn _ready(&mut self, _owner: &gd::Node) {
        self.update();
    }

    #[export]
    pub fn _process(&mut self, _owner: &gd::Node, _delta: f64) {
        self.run_commands(|mut commands| commands.insert_resource(NextState(ProcessState::Idle)));
        self.update();
    }

    #[export]
    pub fn _physics_process(&mut self, _owner: &gd::Node, _delta: f64) {
        self.run_commands(|mut commands| {
            commands.insert_resource(NextState(ProcessState::Physics))
        });
        self.update();
    }
}

//Owner as a Resource

#[derive(Deref, DerefMut)]
pub struct EcsNode(Ref<Node>);

//STATE

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ProcessState {
    Idle,
    Physics,
}

//SYSTEMS

pub fn exit_on_escape(ecs_node: ResMut<EcsNode>) {
    let input = gd::Input::godot_singleton();

    if input.is_action_just_pressed("ui_cancel", false) {
        let node = unsafe { ecs_node.assume_safe() };
        let tree = node
            .get_tree()
            .map(|tree| unsafe { tree.assume_safe() })
            .unwrap();

        tree.quit(-1);
    }
}

pub fn hello_bevy() {
    println!("Hello, Bevy!");
}
