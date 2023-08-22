use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Line {
  x1: usize,
  y1: usize,
  x2: usize,
  y2: usize,
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

fn day5a() -> Result<i32, Box<dyn std::error::Error>> {
  let mut overlap_map = vec![0; 1000 * 1000];
  let mut num_overlaps = 0;
  include_str!("./input")
    .lines()
    .map(|l| Line::from_str(l).unwrap())
    .filter(|l| l.x1 == l.x2 || l.y1 == l.y2) // consider only horz & vert lines
    .for_each(|l| {
      let mut mark = |x, y| {
        if overlap_map[x + y * 1000] == 1 {
          num_overlaps += 1;
        }
        overlap_map[x + y * 1000] += 1;
      };
      if l.x1 == l.x2 {
        (l.y1.min(l.y2)..=l.y1.max(l.y2)).for_each(|y| mark(l.x1, y))
      } else if l.y1 == l.y2 {
        (l.x1.min(l.x2)..=l.x1.max(l.x2)).for_each(|x| mark(x, l.y1))
      }
    });

  Ok(num_overlaps)
}

fn main() {
  println!("{}", day5a().unwrap())
}
