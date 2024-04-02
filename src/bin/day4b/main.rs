#![feature(extract_if)]

use std::collections::HashMap;

type Board = HashMap<u8, (usize, usize, bool)>;

fn day4b() -> Result<i32, Box<dyn std::error::Error>> {
  let (nums, boards) = include_str!("./input").split_once("\n\n").unwrap();

  let mut boards: Vec<(usize, Board)> = boards // index, board
    .split("\n\n")
    .enumerate()
    .map(|(bindex, b)| {
      (
        bindex,
        b.split_ascii_whitespace()
          .enumerate()
          .scan(0, |s, (i, n)| {
            if i % 5 == 0 {
              *s += 1;
            }

            Some((n.parse().unwrap(), (*s - 1, i % 5, false)))
          })
          .collect(),
      )
    })
    .collect();

  let nums: Vec<_> = nums.split(',').map(|n| n.parse().unwrap()).collect();
  let mut marks = vec![([0; 5], [0; 5]); boards.len()];

  let (num, board) = nums
    .iter()
    .filter_map(|n| {
      boards
        .extract_if(|(bi, b)| {
          let bi = *bi;
          b.get_mut(n)
            .map(|(i, j, m)| {
              marks[bi].0[*i] += 1;
              marks[bi].1[*j] += 1;
              *m = true;
              marks[bi].0[*i] == 5 || marks[bi].1[*j] == 5
            })
            .unwrap_or(false)
        })
        .map(|(_, b)| (*n, b))
        .next()
    })
    .last()
    .unwrap();

  let sum: u8 = board.iter().filter(|(_, (_, _, m))| !*m).map(|(n, _)| n).sum();

  Ok(sum as i32 * num as i32)
}

fn main() {
  println!("{}", day4b().unwrap())
}
