use ggez::{
    GameResult
};

pub enum SceneCommand<C> {
    None,
    Push(Box<Scene<Ctx = C>>),
    Swap(Box<Scene<Ctx = C>>),
    Pop
}

pub trait Scene {
    type Ctx;

    fn update(&mut self, context: &mut Self::Ctx) -> SceneCommand<Self::Ctx>;
    fn draw(&mut self, context: &mut Self::Ctx);
    fn overlay(&self) -> bool {
        false
    }
}

pub struct SceneStack<C> {
    pub current_scenes: Vec<Box<Scene<Ctx = C>>>
}
