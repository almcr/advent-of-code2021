pub fn main() {
  let map: Vec<_> = include_bytes!("input").split(|b| b == &b'\n').collect();
  let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

  let mut sum = 0;
  for (i, l) in map.iter().enumerate() {
    for (j, cell) in l.iter().enumerate() {
      if neighbors.iter().all(|(x, y)| {
        map
          .get((i as isize + x) as usize)
          .and_then(|&row| row.get((j as isize + y) as usize).map(|n| cell < n))
          .unwrap_or(true)
      }) {
        sum += usize::from(*cell - b'0') + 1;
      }
    }
  }

  println!("{}", sum as u32)
}
