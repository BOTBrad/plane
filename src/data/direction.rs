#[derive(Copy, PartialEq)]
enum CardinalH {
  Left = -1,
  Neutral = 0,
  Right = 1,
}

#[derive(Copy, PartialEq)]
enum CardinalV {
  Up = -1,
  Neutral = 0,
  Down = 1,
}

// this can't be a type alias
// https://github.com/rust-lang/rust/issues/9767
#[derive(Copy)]
pub struct Direction(CardinalH, CardinalV);

impl CardinalH {
  fn combine(v1: CardinalH, v2: CardinalH) -> CardinalH {
    match (v1 as i8) + (v2 as i8) {
      -2 | -1 => CardinalH::Left,
      0 => CardinalH::Neutral,
      1 | 2 => CardinalH::Right,
      _ => unreachable!(),
    }
  }
}

impl CardinalV {
  fn combine(v1: CardinalV, v2: CardinalV) -> CardinalV {
    match (v1 as i8) + (v2 as i8) {
      -2 | -1 => CardinalV::Up,
      0 => CardinalV::Neutral,
      1 | 2 => CardinalV::Down,
      _ => unreachable!(),
    }
  }
}

const LEFT: Direction = Direction(CardinalH::Left, CardinalV::Neutral);
const DOWN: Direction = Direction(CardinalH::Neutral, CardinalV::Down);
const UP: Direction = Direction(CardinalH::Neutral, CardinalV::Up);
const RIGHT: Direction = Direction(CardinalH::Right, CardinalV::Neutral);
const NEUTRAL: Direction = Direction(CardinalH::Neutral, CardinalV::Neutral);

impl Direction {
  pub fn left() -> Direction {
    LEFT
  }

  pub fn down() -> Direction {
    DOWN
  }

  pub fn up() -> Direction {
    UP
  }

  pub fn right() -> Direction {
    RIGHT
  }

  pub fn neutral() -> Direction {
    NEUTRAL
  }

  pub fn combine(d1: Direction, d2: Direction) -> Direction {
    let Direction(h1, v1) = d1;
    let Direction(h2, v2) = d2;

    Direction(CardinalH::combine(h1, h2), CardinalV::combine(v1, v2))
  }

  pub fn to_normalized_pair(&self, scalar: i32) -> (i32, i32) {
    let Direction(ch, cv) = *self;
    let h = ch as i32;
    let v = cv as i32;

    let pair: (i32, i32) = (scalar * h, scalar * v);

    match pair {
      (0, _)
      | (_, 0) => pair,
      (a, b) => (a * 12 / 17, b * 12 / 17), // divide by sqrt(2) approximation
    }
  }
}

