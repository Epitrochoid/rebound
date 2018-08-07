use ggez;
use ggez::graphics;

use scene;
use world::World;

pub struct StartScene {
    center_text: graphics::Text
}

impl StartScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 60)
            .expect("Could not load /DejaVuSerif.ttf");
        let text = graphics::Text::new(ctx, "Start!", &font)
            .expect("Could not create 'Start!' text");

        Self { center_text: text }
    }
}

impl scene::Scene for StartScene {
    type Ctx = ggez::Context;

    fn update(&mut self, context: &mut Self::Ctx) -> scene::SceneCommand<Self::Ctx> {
        scene::SceneCommand::None
    }

    fn draw(&mut self, context: &mut Self::Ctx) {
        let loc = graphics::Point2::new(100.0, 100.0);
        graphics::draw(context, &self.center_text, loc, 0.0)
            .expect("Could not draw 'Start!' text");
    }
}
