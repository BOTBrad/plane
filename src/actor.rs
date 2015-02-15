use data;
use graphics;
use std;

#[derive(Copy)]
pub enum Direction {
  Left,
  Neutral,
  Right
}

#[derive(PartialEq)]
#[derive(Copy)]
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
      jump_state: JumpState::Standing,
      direction: Direction::Neutral
    }
  }

  pub fn update(&mut self) {
    if self.y >= self.floor {
      // we are grounded
      // we can switch directions when not crouching
      if self.jump_state != JumpState::Crouching {
        match self.direction {
          Direction::Left => self.vx = -self.v,
          Direction::Neutral => self.vx = 0,
          Direction::Right => self.vx = self.v
        };
      }
      // jump if we are trying to
      match self.jump_state {
        JumpState::Crouching => {
          self.vx = 0;
          self.vy = 0
        },
        JumpState::Standing =>
          self.vy = 0,
        JumpState::Jumping =>
          self.vy = -self.jump_v,
      };
    } else {
      // we are airborne
      // apply gravity
      self.vy += self.gravity;
    }
    // we always move
    self.x += self.vx;
    // don't fall through the floor
    self.y = std::cmp::min(self.floor, self.y + self.vy);
  }

  pub fn change_dir(&mut self, direction: Direction) {
    match (self.direction, direction) {
      (Direction::Neutral, _) => self.direction = direction,
      (Direction::Left, Direction::Right)
      | (Direction::Right, Direction::Left) => self.direction = Direction::Neutral,
      _ => {}
    };
  }

  pub fn change_jump_state(&mut self, jump_state: JumpState) {
    match (self.jump_state, jump_state) {
      (JumpState::Standing, _) => self.jump_state = jump_state,
      (JumpState::Jumping, JumpState::Crouching)
      | (JumpState::Crouching, JumpState::Jumping) => self.jump_state = JumpState::Standing,
      _ => {}
    };
  }
}

impl graphics::Renderable for Actor {
  fn render(&self) -> data::Rect {
    let x = self.x;
    let y = self.y;
    let w = 48;
    let h =
      if self.jump_state == JumpState::Crouching {
        64
      } else {
        128
      };

    data::Rect {
      x: x - w/2,
      y: y - h,
      w: w,
      h: h
    }
  }
}

