use ggez::{
    GameResult
}

pub trait Scene {
    type Ctx;
    type Event;

    fn update(&mut self, context: &mut Ctx) -> GameResult<()>;
    fn draw(&mut self, context: &mut Ctx);
    fn overlay(&self) -> bool {
        false
    }
}

pub struct SceneStack {
    pub current_scenes: Vec<Scene>
}
