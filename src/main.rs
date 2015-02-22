extern crate sdl2;

mod actor;
mod data;
mod game_state;
mod global;
mod graphics;
mod timer;

fn main() {
  let graphics = graphics::Graphics::new(global::SCREEN_SIZE);
  let mut game_state = game_state::GameState::new();
  let mut timer = timer::Timer::new();

  'main: loop {
    while !timer.is_next_frame() {}
    // update and render next frame
    match game_state.next_frame() {
      game_state::GameStateUpdate::Quit => break,
      game_state::GameStateUpdate::Render(render_list) => graphics.update(render_list),
    };
  }
  graphics.quit();
}

