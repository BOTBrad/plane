extern crate sdl2;

use actor;
use graphics;
use std;

pub struct GameState {
  color: sdl2::pixels::Color,
  character: actor::Actor
}

pub enum GameStateUpdate {
  Render(std::vec::Vec<graphics::Rect>, sdl2::pixels::Color),
  Quit
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      color: sdl2::pixels::RGB(0, 0, 0),
      character: actor::Actor::new(100, 600, 3, 40, 12)
    }
  }

  pub fn next_frame(&mut self) -> GameStateUpdate {
    'event: loop {
      match sdl2::event::poll_event() {
        sdl2::event::QuitEvent(_) => return Quit,
        sdl2::event::KeyDownEvent(_, _, key, _, _) => match key {
          sdl2::keycode::EscapeKey
          | sdl2::keycode::QKey => return Quit,
          sdl2::keycode::Num1Key => self.color = sdl2::pixels::RGB(255, 0, 0),
          sdl2::keycode::Num2Key => self.color = sdl2::pixels::RGB(0, 255, 0),
          sdl2::keycode::Num3Key => self.color = sdl2::pixels::RGB(0, 0, 255),
          sdl2::keycode::WKey => self.character.change_jump_state(actor::Jumping),
          sdl2::keycode::SKey => self.character.change_jump_state(actor::Crouching),
          sdl2::keycode::AKey => self.character.change_dir(actor::Left),
          sdl2::keycode::DKey => self.character.change_dir(actor::Right),
          _ => {}
        },
        sdl2::event::KeyUpEvent(_, _, key, _, _) => match key {
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
    Render(render_list, self.color.clone())
  }
}
