use graphics;

pub enum Direction {
  Left,
  Neutral,
  Right
}

#[deriving(PartialEq)]
pub enum JumpState {
  Crouching,
  Standing,
  Jumping,
}

pub struct Actor {
  pub x: i32,
  pub y: i32,
  floor: i32,
  gravity: i32,
  jump_v: i32,
  v: i32,
  vx: i32,
  vy: i32,
  jump_state: JumpState,
  direction: Direction
}

impl Actor {
  pub fn new(x: i32, floor: i32, gravity: i32, jump_v: i32, v: i32) -> Actor {
    Actor {
      x: x,
      y: floor,
      floor: floor,
      gravity: gravity,
      jump_v: jump_v,
      v: v,
      vx: 0,
      vy: 0,
      jump_state: Standing,
      direction: Neutral
    }
  }

  pub fn update(&mut self) {
    if self.y >= self.floor {
      // we are grounded
      // don't fall through the floor
      self.y = self.floor;
      // we can switch directions when not crouching
      if self.jump_state != Crouching {
        match self.direction {
          Left => self.vx = -self.v,
          Neutral => self.vx = 0,
          Right => self.vx = self.v
        };
      }
      // jump if we are trying to
      match self.jump_state {
        Crouching => {
          self.vx = 0;
          self.vy = 0
        },
        Standing =>
          self.vy = 0,
        Jumping =>
          self.vy = -self.jump_v,
      };
    } else {
      // we are airborne
      // apply gravity
      self.vy += self.gravity;
    }
    // we always move
    self.x += self.vx;
    self.y += self.vy;
  }

  pub fn change_dir(&mut self, direction: Direction) {
    match (self.direction, direction) {
      (Neutral, _) => self.direction = direction,
      (Left, Right)
      | (Right, Left) => self.direction = Neutral,
      _ => {}
    };
  }

  pub fn change_jump_state(&mut self, jump_state: JumpState) {
    match (self.jump_state, jump_state) {
      (Standing, _) => self.jump_state = jump_state,
      (Jumping, Crouching)
      | (Crouching, Jumping) => self.jump_state = Standing,
      _ => {}
    };
  }
}

impl graphics::Renderable for Actor {
  fn render(&self) -> graphics::Rect {
    let x = self.x;
    let y = self.y;
    let w = 48;
    let h =
      if self.jump_state == Crouching {
        64
      } else {
        128
      };

    graphics::Rect {
      x: x - w/2,
      y: y - h,
      w: w,
      h: h
    }
  }
}
