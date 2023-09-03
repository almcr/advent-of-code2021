const ADJ: [(isize, isize); 8] = [
  (0, -1),
  (1, -1),
  (1, 0),
  (1, 1),
  (0, 1),
  (-1, 1),
  (-1, 0),
  (-1, -1),
];

fn flash(m: &mut Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
  m[x][y] = b'0';
  ADJ
    .iter()
    .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
    .fold(1, |acc, (x, y)| {
      match m.get_mut(x).and_then(|l| l.get_mut(y)) {
        Some(n) if *n > b'0' => {
          *n += 1;
          acc + (*n > b'9').then(|| flash(m, x, y)).unwrap_or(0)
        }
        _ => acc,
      }
    })
}

pub fn main() {
  let mut m = include_bytes!("input")
    .split(|&b| b == b'\n')
    .map(|l| l.to_vec())
    .collect::<Vec<_>>();

  let flashes = (0..100).fold(0, |acc, _| {
    m.iter_mut()
      .for_each(|l| l.iter_mut().for_each(|n| *n += 1));

    let mut s = 0;
    for i in 0..10 {
      for j in 0..10 {
        s += (m[i][j] > b'9').then(|| flash(&mut m, i, j)).unwrap_or(0);
      }
    }
    acc + s
  });

  println!("{}", flashes);
}
