use data;
use graphics;

#[derive(Copy)]
pub enum Direction {
  Left,
  Neutral,
  Right
}

pub struct Actor {
  pub x: i32,
  pub y: i32,
  v: i32,
  vx: i32,
  direction: Direction
}

impl Actor {
  pub fn new(x: i32, y: i32, v: i32) -> Actor {
    Actor {
      x: x,
      y: y,
      v: v,
      vx: 0,
      direction: Direction::Neutral
    }
  }

  pub fn update(&mut self) {
    self.vx = match self.direction {
      Direction::Left => -self.v,
      Direction::Neutral => 0,
      Direction::Right => self.v,
    };
    // we always move
    self.x += self.vx;
  }

  pub fn change_dir(&mut self, direction: Direction) {
    match (self.direction, direction) {
      (Direction::Neutral, _) => self.direction = direction,
      (Direction::Left, Direction::Right)
      | (Direction::Right, Direction::Left) => self.direction = Direction::Neutral,
      _ => {}
    };
  }
}

impl graphics::Renderable for Actor {
  fn render(&self) -> data::Rect {
    let x = self.x;
    let y = self.y;
    let w = 48;
    let h = 64;

    data::Rect {
      x: x - w/2,
      y: y - h,
      w: w,
      h: h
    }
  }
}

