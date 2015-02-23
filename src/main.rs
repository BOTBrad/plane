#![feature(core)]
extern crate sdl2;

use graphics::Graphics;
use game_state::{GameState, GameStateUpdate};
use timer::Timer;

mod actor;
mod data;
mod game_state;
mod global;
mod graphics;
mod timer;

fn main() {
  let graphics = Graphics::new(global::SCREEN_SIZE);
  let mut game_state = GameState::new();
  let mut timer = Timer::new();

  'main: loop {
    while !timer.is_next_frame() {}
    // update and render next frame
    match game_state.next_frame() {
      GameStateUpdate::Quit => break,
      GameStateUpdate::Render(render_list) => graphics.update(render_list),
    };
  }
  graphics.quit();
}

