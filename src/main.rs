extern crate ggez;
extern crate specs;

#[macro_use]
extern crate specs_derive;

use ggez::{
    conf,
    event,
    graphics,
    timer,
    Context,
    ContextBuilder,
    GameResult,
    nalgebra::{
        Vector2
    }
};

use specs::{
    Entity,
    World
};

use std::{
    env,
    path
};

mod components;
mod drawables;
mod scene;
mod scenes;
mod world;

struct GameState {
    specs_world: World
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut world = World::new();
        world.register::<components::Position>();
        world.register::<components::Kinematics>();
        world.register::<components::Draw>();

        let ball = drawables::Ball { radius: 10.0, blend_mode: Some(graphics::BlendMode::Alpha) };

        world.create_entity()
            .with(components::Position(Vector2::new(100.0, 100.0)))
            .with(components::Kinematics { velocity: Vector2::new(0.0, 0.0),
                                           acceleration: Vector2::new(0.0, 0.0)})
            .with(components::Draw(ball))
            .build();

        let state = GameState { specs_world: world };
        Ok(state)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;

        let drawings = self.specs_world.read_storage::<components::Draw>();
        let positions = self.specs_world.read_storage::<components::Position>();

        graphics::clear(ctx);

        for (draw, pos) in (&drawings, &positions).join() {
            graphics::draw(ctx, &draw.0, graphics::Point2::new(pos.0.x, pos.0.y), 0.0)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut ggez_conf = ContextBuilder::new("rebound", "rebound")
        .window_setup(conf::WindowSetup::default().title("Rebound"))
        .window_mode(conf::WindowMode::default().dimensions(1200, 1200));

    let ctx = &mut ggez_conf.build().unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut GameState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error: {}", e);
    } else {
        println!("Exited.");
    }
}
