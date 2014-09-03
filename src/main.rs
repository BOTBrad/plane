extern crate sdl2;

mod actor;
mod global;
mod graphics;
mod timer;

fn main() {
  let graphics = graphics::Graphics::new(global::SCREEN_SIZE);

  let mut timer = timer::Timer::new();
  let mut color = sdl2::pixels::RGB(0, 0, 0);
  let mut character = actor::Actor::new(100, 600, 3, 40, 12);

  'main: loop {
    while !timer.is_next_frame() {}
    'event: loop {
      match sdl2::event::poll_event() {
        sdl2::event::QuitEvent(_) => break 'main,
        sdl2::event::KeyDownEvent(_, _, key, _, _) => match key {
          sdl2::keycode::EscapeKey | sdl2::keycode::QKey => break 'main,
          sdl2::keycode::Num1Key => color = sdl2::pixels::RGB(255, 0, 0),
          sdl2::keycode::Num2Key => color = sdl2::pixels::RGB(0, 255, 0),
          sdl2::keycode::Num3Key => color = sdl2::pixels::RGB(0, 0, 255),
          sdl2::keycode::WKey => character.change_jump_state(actor::Jumping),
          sdl2::keycode::SKey => character.change_jump_state(actor::Crouching),
          sdl2::keycode::AKey => character.change_dir(actor::Left),
          sdl2::keycode::DKey => character.change_dir(actor::Right),
          _ => {}
        },
        sdl2::event::KeyUpEvent(_, _, key, _, _) => match key {
          sdl2::keycode::WKey => character.change_jump_state(actor::Crouching),
          sdl2::keycode::SKey => character.change_jump_state(actor::Jumping),
          sdl2::keycode::AKey => character.change_dir(actor::Right),
          sdl2::keycode::DKey => character.change_dir(actor::Left),
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
