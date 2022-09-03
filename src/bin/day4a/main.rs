use std::collections::HashMap;

fn day4a() -> Result<i32, Box<dyn std::error::Error>> {
  let (nums, boards) = include_str!("./input").split_once("\n\n").unwrap();

  let mut boards: Vec<HashMap<i32, (usize, usize, bool)>> = boards
    .split("\n\n")
    .map(|b| {
      b.split_ascii_whitespace()
        .enumerate()
        .scan(0, |s, (i, n)| {
          if i % 5 == 0 {
            *s += 1;
          }

          Some((n.parse().unwrap(), (*s - 1, i % 5, false)))
        })
        .collect()
    })
    .collect();

  let nums: Vec<i32> = nums.split(',').map(|n| n.parse().unwrap()).collect();
  let mut marks = vec![([0; 5], [0; 5]); boards.len()];

  let (board, bn) = nums
    .iter()
    .find_map(|n| {
      boards.iter_mut().enumerate().find_map(|(bi, b)| {
        b.get_mut(n)
          .map(|(i, j, m)| {
            marks[bi].0[*i] += 1;
            marks[bi].1[*j] += 1;
            *m = true;
            (*i, *j)
          })
          .filter(|(i, j)| marks[bi].0[*i] == 5 || marks[bi].1[*j] == 5)
          .map(|_| (bi, n))
      })
    })
    .unwrap();

  let sum = boards[board]
    .iter()
    .filter(|(_, (_, _, m))| !*m)
    .map(|(n, _)| n)
    .sum::<i32>();

  Ok(sum * bn)
}

fn main() {
  println!("{}", day4a().unwrap())
}
