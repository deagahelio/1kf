use ggez;
use ggez::event;
use ggez::GameResult;
use okf::states::game_state::GameState;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("1kf", "deagahelio");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new();
    event::run(ctx, event_loop, state)
}