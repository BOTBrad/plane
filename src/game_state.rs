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
        sdl2::event::Event::Quit{..} => return GameStateUpdate::Quit,
        sdl2::event::Event::KeyDown{keycode: key, repeat: false, ..} => match key {
          sdl2::keycode::KeyCode::Escape
          | sdl2::keycode::KeyCode::Q => return GameStateUpdate::Quit,
          sdl2::keycode::KeyCode::W => self.character.change_jump_state(actor::JumpState::Jumping),
          sdl2::keycode::KeyCode::S => self.character.change_jump_state(actor::JumpState::Crouching),
          sdl2::keycode::KeyCode::A => self.character.change_dir(actor::Direction::Left),
          sdl2::keycode::KeyCode::D => self.character.change_dir(actor::Direction::Right),
          _ => {}
        },
        sdl2::event::Event::KeyUp{keycode: key, repeat: false, ..} => match key {
          sdl2::keycode::KeyCode::W => self.character.change_jump_state(actor::JumpState::Crouching),
          sdl2::keycode::KeyCode::S => self.character.change_jump_state(actor::JumpState::Jumping),
          sdl2::keycode::KeyCode::A => self.character.change_dir(actor::Direction::Right),
          sdl2::keycode::KeyCode::D => self.character.change_dir(actor::Direction::Left),
          _ => {}
        },
        sdl2::event::Event::None => break,
        _ => {}
      }
    }
    self.character.update();

    let render_list = vec![(&self.character as &graphics::Renderable).render()];
    GameStateUpdate::Render(render_list)
  }
}

