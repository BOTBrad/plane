extern crate sdl2;

use sdl2::event::{self, Event};
use sdl2::keycode::KeyCode;

use actor::Actor;
use data::Direction;
use graphics::{self, RenderList};
use std::vec::Vec;

pub struct GameState {
  character: Actor
}

pub enum GameStateUpdate {
  Render(Vec<RenderList>),
  Quit,
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      character: Actor::new(100, 600, 6)
    }
  }

  pub fn next_frame(&mut self) -> GameStateUpdate {
    'event: loop {
      match event::poll_event() {
        Event::Quit{..} => return GameStateUpdate::Quit,
        Event::KeyDown{keycode: key, repeat: false, ..} => match key {
          KeyCode::Escape
          | KeyCode::Q => return GameStateUpdate::Quit,
          KeyCode::W => self.character.change_dir(Direction::up()),
          KeyCode::S => self.character.change_dir(Direction::down()),
          KeyCode::A => self.character.change_dir(Direction::left()),
          KeyCode::D => self.character.change_dir(Direction::right()),
          _ => {},
        },
        Event::KeyUp{keycode: key, repeat: false, ..} => match key {
          KeyCode::W => self.character.change_dir(Direction::down()),
          KeyCode::S => self.character.change_dir(Direction::up()),
          KeyCode::A => self.character.change_dir(Direction::right()),
          KeyCode::D => self.character.change_dir(Direction::left()),
          _ => {},
        },
        Event::None => break,
        _ => {},
      }
    }
    self.character.update();

    let render_list = vec![(&self.character as &graphics::Renderable).render()];
    GameStateUpdate::Render(render_list)
  }
}

