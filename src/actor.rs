use data::{Rect, Direction};
use graphics;

pub struct Actor {
  pub x: i32,
  pub y: i32,
  v: i32,
  vx: i32,
  direction: Direction,
}

impl Actor {
  pub fn new(x: i32, y: i32, v: i32) -> Actor {
    Actor {
      x: x,
      y: y,
      v: v,
      vx: 0,
      direction: Direction::neutral(),
    }
  }

  pub fn update(&mut self) {
    let (vx, vy) = self.direction.to_normalized_pair(self.v);
    self.x += vx;
    self.y += vy;
  }

  pub fn change_dir(&mut self, direction: Direction) {
    self.direction = Direction::combine(self.direction, direction);
  }
}

impl graphics::Renderable for Actor {
  fn render(&self) -> Rect {
    let x = self.x;
    let y = self.y;
    let w = 48;
    let h = 64;

    Rect {
      x: x - w/2,
      y: y - h,
      w: w,
      h: h,
    }
  }
}

