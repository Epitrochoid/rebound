use ggez;
use specs;

use components;
use scene;

pub struct World {
    pub specs_world: specs::World,
    pub scenes: scene::SceneStack
}

impl World {
    fn register_components(&mut self) {
        self.specs_world.register::<components::Position>();
        self.specs_world.register::<components::Kinematics>();
        self.specs_world.register::<components::Draw>();
    }

    fn new(ctx: &mut ggez::Context) -> Self {
        let specs_world = specs::World::new();
        let scenes = scene::SceneStack::new();

        { specs_world, scenes }
    }
}
