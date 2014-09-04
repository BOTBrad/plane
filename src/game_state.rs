extern crate sdl2;

use actor;
use data;
use graphics;
use std;

pub struct GameState {
  character: actor::Actor
}

pub enum GameStateUpdate {
  Render(std::vec::Vec<data::Rect>),
  Quit
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      character: actor::Actor::new(100, 600, 3, 40, 12)
    }
  }

  pub fn next_frame(&mut self) -> GameStateUpdate {
    'event: loop {
      match sdl2::event::poll_event() {
        sdl2::event::QuitEvent(_) => return Quit,
        sdl2::event::KeyDownEvent(_, _, key, false, _, _) => match key {
          sdl2::keycode::EscapeKey
          | sdl2::keycode::QKey => return Quit,
          sdl2::keycode::WKey => self.character.change_jump_state(actor::Jumping),
          sdl2::keycode::SKey => self.character.change_jump_state(actor::Crouching),
          sdl2::keycode::AKey => self.character.change_dir(actor::Left),
          sdl2::keycode::DKey => self.character.change_dir(actor::Right),
          _ => {}
        },
        sdl2::event::KeyUpEvent(_, _, key, false, _, _) => match key {
          sdl2::keycode::WKey => self.character.change_jump_state(actor::Crouching),
          sdl2::keycode::SKey => self.character.change_jump_state(actor::Jumping),
          sdl2::keycode::AKey => self.character.change_dir(actor::Right),
          sdl2::keycode::DKey => self.character.change_dir(actor::Left),
          _ => {}
        },
        sdl2::event::NoEvent => break,
        _ => {}
      }
    }
    self.character.update();

    let render_list = vec![(&self.character as &graphics::Renderable).render()];
    Render(render_list)
  }
}
