extern crate sdl2;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;

use actor::Actor;
use data::{Direction, Rect};
use graphics;
use std::vec::Vec;

pub struct GameState {
  character: Actor
}

pub enum GameStateUpdate {
  Render(Vec<Rect>),
  Quit,
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      character: Actor::new(100, 600, 3)
    }
  }

  pub fn next_frame(&mut self) -> GameStateUpdate {
    'event: loop {
      match sdl2::event::poll_event() {
        Event::Quit{..} => return GameStateUpdate::Quit,
        Event::KeyDown{keycode: key, repeat: false, ..} => match key {
          KeyCode::Escape
          | KeyCode::Q => return GameStateUpdate::Quit,
          KeyCode::W => (),
          KeyCode::S => (),
          KeyCode::A => self.character.change_dir(Direction::Left),
          KeyCode::D => self.character.change_dir(Direction::Right),
          _ => {},
        },
        Event::KeyUp{keycode: key, repeat: false, ..} => match key {
          KeyCode::W => (),
          KeyCode::S => (),
          KeyCode::A => self.character.change_dir(Direction::Right),
          KeyCode::D => self.character.change_dir(Direction::Left),
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

