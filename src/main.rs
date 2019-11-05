use ggez;
use ggez::event;
use ggez::GameResult;
use okf::states::game_state::GameState;
use okf::generator::SevenBagGenerator;

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("1kf", "deagahelio")
        .window_setup(ggez::conf::WindowSetup::default().title("1kf"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(576.0, 704.0))
        .build()?;
    let state = &mut GameState::new(SevenBagGenerator::new());
    event::run(ctx, event_loop, state)
}