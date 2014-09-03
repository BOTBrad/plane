extern crate sdl2;

use std;

pub struct Timer {
  next_frame: (uint, u8)
}

impl Timer {
  pub fn new() -> Timer {
    Timer {
      next_frame: (0, 0)
    }
  }

  // hardcoded to 60fps
  pub fn is_next_frame(&mut self) -> bool {
    let ticks = sdl2::timer::get_ticks();
    let (time, rounding) = self.next_frame;

    if ticks >= time {
      let new_time = std::cmp::max(
        ticks,
        match rounding {
          0 | 1 => time + 17,
          _ => time + 16
        });
      self.next_frame = (new_time, (rounding + 1) % 3);
      true
    } else {
      false
    }
  }
}
