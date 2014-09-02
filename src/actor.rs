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
