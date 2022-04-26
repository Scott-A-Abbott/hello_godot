use bevy::prelude::*;
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
        //Inserts the default thread pools into world
        DefaultTaskPoolOptions::default().create_default_pools(&mut app.world);

        //CoreStages include: First, PreUpdate, Update, PostUpdate, Last
        app.add_default_stages()
            .add_system_to_stage(CoreStage::Last, World::clear_trackers.exclusive_system());

        Self(app)
    }
}

#[methods]
impl Ecs {
    pub fn new(owner: TRef<gd::Node>) -> Self {
        let mut ecs = Self::default();

        ecs.add_loopless_state(ProcessState::Idle)
            .insert_resource(EcsNode(owner.claim()))
            .insert_resource(IdleDelta::default())
            .insert_resource(PhysicsDelta::default())
            .add_startup_system(hello_bevy)
            .add_system(exit_on_escape.run_in_state(ProcessState::Idle));

        ecs
    }

    #[export]
    pub fn _ready(&mut self, _owner: &gd::Node) {
        self.update();
    }

    #[export]
    pub fn _process(&mut self, _owner: &gd::Node, delta: f64) {
        self.world.insert_resource(NextState(ProcessState::Idle));
        self.world.insert_resource(IdleDelta(delta));

        self.update();
    }

    #[export]
    pub fn _physics_process(&mut self, _owner: &gd::Node, delta: f64) {
        self.world.insert_resource(NextState(ProcessState::Physics));
        self.world.insert_resource(PhysicsDelta(delta));

        self.update();
    }
}

// RESOURCES

#[derive(Deref, DerefMut)]
pub struct EcsNode(Ref<Node>);

#[derive(Deref, DerefMut, Default)]
pub struct IdleDelta(f64);

#[derive(Deref, DerefMut, Default)]
pub struct PhysicsDelta(f64);

// STATE

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ProcessState {
    Idle,
    Physics,
}

// SYSTEMS

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
