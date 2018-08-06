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
