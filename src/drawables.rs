use ggez::{
    Context,
    GameResult,
    graphics::{
        BlendMode,
        circle,
        Color,
        Drawable,
        DrawMode,
        DrawParam,
        set_color
    }
};


pub struct Ball {
    pub radius: f32,
    pub blend_mode: Option<BlendMode>
}

impl Drawable for Ball {
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0));
        circle(ctx, DrawMode::Fill, param.dest, self.radius, 2.0)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.blend_mode = mode;
    }

    fn get_blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }
}
