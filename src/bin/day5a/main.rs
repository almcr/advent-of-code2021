use std::{num::ParseIntError, str::FromStr};

struct Line {
  x1: u32,
  y1: u32,
  x2: u32,
  y2: u32,
}

impl FromStr for Line {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (p1, p2) = s.split_once(" -> ").unwrap();
    let (x1, y1) = p1
      .split_once(',')
      .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
      .unwrap();
    let (x2, y2) = p2
      .split_once(',')
      .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
      .unwrap();
    Ok(Self { x1, y1, x2, y2 })
  }
}

impl Line {
  fn intersect(&self, rhs: &Self) -> (u32, u32) {
    if rhs.x1 >= self.x1 && self.x1 <= rhs.x2 {
      
    }
  }
}

fn day5a() -> Result<i32, Box<dyn std::error::Error>> {
  let lines: Vec<Line> = include_str!("./test")
    .lines()
    .map(|l| Line::from_str(l).unwrap())
    .collect();
  
  
  Ok(0)
}

fn main() {
  println!("{}", day5a().unwrap())
}

#[test]
fn intersect_test() {
  let l1 = Line::from_str("0,0 -> 2,0");
  let l2 = Line::from_str("1,0 -> 1,2");
}