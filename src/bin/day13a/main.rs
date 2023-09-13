fn main() {
  let (coords, folds) = include_str!("input").split_once("\n\n").unwrap();
  let folds: Vec<(char, i32)> = folds
    .lines()
    .map(|l| l.split_once('='))
    .map(|s| {
      (
        s.unwrap().0.chars().last().unwrap(),
        s.unwrap().1.parse::<i32>().unwrap(),
      )
    })
    .collect();

  let (fold, valx) = folds[0];
  let valy = i32::MAX;

  let mut coords = coords
    .lines()
    .map(|l| {
      l.split_once(',')
        .map(|c| (c.0.parse::<i32>().unwrap(), c.1.parse::<i32>().unwrap()))
        .unwrap()
    })
    .map(|(x, y)| (if x > valx { (valx * 2) - x } else { x }, y))
    .collect::<Vec<_>>();
  coords.sort_unstable();
  coords.dedup();

  println!("{:?}", coords.len());
}
