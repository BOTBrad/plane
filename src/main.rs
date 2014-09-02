extern crate sdl2;

mod actor;
mod graphics;

fn main() {
  let graphics = graphics::Graphics::new(800, 600);

  let mut color = sdl2::pixels::RGB(0, 0, 0);
  let mut character = actor::Actor {
    x: 100,
    y: 100,
    vx: 0,
    vy: 0
  };

  'main: loop {
    'event: loop {
      match sdl2::event::poll_event() {
        sdl2::event::QuitEvent(_) => break 'main,
        sdl2::event::KeyDownEvent(_, _, key, _, _) => match key {
          sdl2::keycode::EscapeKey | sdl2::keycode::QKey => break 'main,
          sdl2::keycode::Num1Key => color = sdl2::pixels::RGB(255, 0, 0),
          sdl2::keycode::Num2Key => color = sdl2::pixels::RGB(0, 255, 0),
          sdl2::keycode::Num3Key => color = sdl2::pixels::RGB(0, 0, 255),
          sdl2::keycode::WKey => character.vy = -1,
          sdl2::keycode::SKey => character.vy = 1,
          sdl2::keycode::AKey => character.vx = -1,
          sdl2::keycode::DKey => character.vx = 1,
          _ => {}
        },
        sdl2::event::NoEvent => break 'event,
        _ => {}
      }
    }
    // render
    character.update();
    graphics.update(&character, &color);
  }
  graphics.quit();
}
