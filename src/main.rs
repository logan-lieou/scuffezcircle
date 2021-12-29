use ggez::*;

struct State {}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: 200.0, y: 300.0},
            100.0,
            0.1,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State {};
    let cb = ggez::ContextBuilder::new("generative_art", "awesome_person");
    let (ctx, event_loop) = cb.build().unwrap();
    event::run(ctx, event_loop, state);
}
