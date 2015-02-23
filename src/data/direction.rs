use std::num::FromPrimitive;

#[derive(Copy, PartialEq, FromPrimitive)]
enum CardinalH {
  Left = -1,
  Neutral = 0,
  Right = 1,
}

#[derive(Copy, PartialEq, FromPrimitive)]
enum CardinalV {
  Up = -1,
  Neutral = 0,
  Down = 1,
}

// this can't be a type alias
// https://github.com/rust-lang/rust/issues/9767
#[derive(Copy)]
pub struct Direction(CardinalH, CardinalV);

impl CardinalV {
  fn combine(v1: CardinalV, v2: CardinalV) -> CardinalV {
    if v1 == v2 {
      v1
    } else {
      FromPrimitive::from_i8(v1 as i8 + v2 as i8).unwrap_or(CardinalV::Neutral)
    }
  }
}

impl CardinalH {
  fn combine(h1: CardinalH, h2: CardinalH) -> CardinalH {
    if h1 == h2 {
      h1
    } else {
      FromPrimitive::from_i8(h1 as i8 + h2 as i8).unwrap_or(CardinalH::Neutral)
    }
  }
}

const LEFT: Direction = Direction(CardinalH::Left, CardinalV::Neutral);
const DOWN: Direction = Direction(CardinalH::Neutral, CardinalV::Down);
const UP: Direction = Direction(CardinalH::Neutral, CardinalV::Up);
const RIGHT: Direction = Direction(CardinalH::Right, CardinalV::Neutral);
const NEUTRAL: Direction = Direction(CardinalH::Neutral, CardinalV::Neutral);

impl Direction {
  // I wish these could be Direction::Left or something
  pub fn left() -> Direction { LEFT }
  pub fn down() -> Direction { DOWN }
  pub fn up() -> Direction { UP }
  pub fn right() -> Direction { RIGHT }
  pub fn neutral() -> Direction { NEUTRAL }

  pub fn combine(d1: Direction, d2: Direction) -> Direction {
    let Direction(h1, v1) = d1;
    let Direction(h2, v2) = d2;

    Direction(CardinalH::combine(h1, h2), CardinalV::combine(v1, v2))
  }

  pub fn components(&self) -> (Direction, Direction) {
    let Direction(h, v) = *self;

    (Direction(CardinalH::Neutral, v), Direction(h, CardinalV::Neutral))
  }

  pub fn to_normalized_pair(&self, scalar: i32) -> (i32, i32) {
    let Direction(ch, cv) = *self;
    let h = ch as i32;
    let v = cv as i32;

    let pair: (i32, i32) = (scalar * h, scalar * v);

    match pair {
      (0, _)
      | (_, 0) => pair,
      (a, b) => (a * 12 / 17, b * 12 / 17),
    }
  }
}

