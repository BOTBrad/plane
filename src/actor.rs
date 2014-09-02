use graphics;

pub struct Actor {
  pub x: i32,
  pub y: i32,
  pub vx: i32,
  pub vy: i32
}

impl Actor {
  pub fn update(&mut self) {
    self.x += self.vx;
    self.y += self.vy;
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
