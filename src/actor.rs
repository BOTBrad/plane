use graphics;

pub enum Direction {
  Left,
  Neutral,
  Right
}

pub struct Actor {
  pub x: i32,
  pub y: i32,
  pub v: i32,
  pub direction: Direction,
  pub vy: i32
}

impl Actor {
  pub fn update(&mut self) {
    match self.direction {
      Left => self.x -= self.v,
      Neutral => {},
      Right => self.x += self.v
    };
  }
  pub fn change_dir(&mut self, direction: Direction) {
    match (self.direction, direction) {
      (Neutral, _) => self.direction = direction,
      (Left, Right)
      | (Right, Left) => self.direction = Neutral,
      _ => {}
    };
  }
}

impl graphics::Renderable for Actor {
  fn render(&self) -> graphics::Rect {
    graphics::Rect {
      x: self.x - 32,
      y: self.y - 64,
      w: 64,
      h: 64
    }
  }
}
