use std::collections::HashMap;

use glam::{UVec2, Vec2};

pub trait BitMask {
  fn init(size: UVec2) -> Self;
  #[must_use]
  fn get_in_direction_from(&self, from: UVec2, direction: Vec2) -> bool;

  fn get_bitmask(&self, position: UVec2) -> Result<i32, String>;

  fn get_top_from(&self, position: UVec2) -> i32;
  fn get_left_from(&self, position: UVec2) -> i32;
  fn get_right_from(&self, position: UVec2) -> i32;
  fn get_bottom_from(&self, position: UVec2) -> i32;
}

impl BitMask for HashMap<(u32, u32), bool> {
  fn init(size: UVec2) -> Self {
    Self::from_iter(
      (0..size.x)
        .flat_map(|x| (0..size.y).map(move |y| ((x, y), false)))
        .collect::<Vec<((u32, u32), bool)>>(),
    )
  }

  fn get_in_direction_from(&self, from: UVec2, direction: Vec2) -> bool {
    let key = from.as_vec2() + direction;
    match self.get(&(key.x as u32, key.y as u32)) {
      Some(value) => *value,
      None => false,
    }
  }

  fn get_top_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: 0., y: 1. }) as i32
  }

  fn get_left_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: -1., y: 0. }) as i32
  }

  fn get_right_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: 1., y: 0. }) as i32
  }

  fn get_bottom_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: 0., y: -1. }) as i32
  }

  fn get_bitmask(&self, position: UVec2) -> Result<i32, String> {
    self
      .get(&(position.x, position.y))
      .map(|_| {
        self.get_top_from(position)
          + 2 * self.get_left_from(position)
          + 4 * self.get_right_from(position)
          + 8 * self.get_bottom_from(position)
      })
      .ok_or("asdasd".into())
  }

  // fn get_left(x: u32, y: u32) -> bool {
  //   false
  // }
}

// impl BitMask for Vec<Vec<bool>> {
//   fn get_in_direction_from(&self, from: UVec2, direction: UVec2) -> bool {
//     false
//   }
// }

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use glam::{UVec2, Vec2};
  use rstest::rstest;

  use crate::BitMask;

  #[test]
  fn init() {
    let map = HashMap::init(UVec2 { x: 2, y: 2 });
    assert_eq!(
      map,
      HashMap::from([
        ((0, 0), false,),
        ((0, 1), false,),
        ((1, 0), false,),
        ((1, 1), false,),
      ])
    )
  }

  #[test]
  fn get_in_direction_from() {
    let mut map = HashMap::<(u32, u32), bool>::init(UVec2 { x: 2, y: 2 });
    map.insert((0, 0), true);
    assert_eq!(
      map.get_in_direction_from(UVec2::new(0, 0), Vec2::new(0., 0.)),
      true
    );
  }

  #[rstest]
  #[case(vec![
  ((0, 0), false), ((1, 0), false), ((2, 0), false),
  ((0, 1), false), ((1, 1), false), ((2, 1), false),
  ((0, 2), false), ((1, 2), false), ((2, 2), false),
  ], 0)]
  #[case(vec![
  ((0, 0), false), ((1, 0), true), ((2, 0), false),
  ((0, 1), false), ((1, 1), false), ((2, 1), false),
  ((0, 2), false), ((1, 2), false), ((2, 2), false),
  ], 8)]
  #[case(vec![
  ((0, 0), false), ((1, 0), true), ((2, 0), false),
  ((0, 1), true), ((1, 1), false), ((2, 1), true),
  ((0, 2), false), ((1, 2), true), ((2, 2), false),
  ], 15)]
  fn get_bitmask(#[case] map: Vec<((u32, u32), bool)>, #[case] res: i32) {
    let map = HashMap::from_iter(map);
    let value = map.get_bitmask(UVec2 { x: 1, y: 1 }).unwrap();
    assert_eq!(value, res);
  }
}
