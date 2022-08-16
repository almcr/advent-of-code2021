const WIDTH: usize = 12;

// check if the bit in pos is set.
fn is_set(num: &i32, pos: i32) -> bool {
  (num & (1 << pos)) != 0
}

fn recurse(pos: i32, bites: Vec<i32>, cmp: fn(&usize, &usize) -> bool) -> i32 {
  if bites.len() == 1 {
    return *bites.last().unwrap();
  }

  let most = cmp(
    &bites.iter().filter(|x| is_set(x, pos)).count(),
    &((bites.len() + 1) / 2),
  );
  recurse(
    pos - 1,
    bites.into_iter().filter(|x| is_set(x, pos) == most).collect(),
    cmp,
  )
}

fn day3a() -> Result<i32, Box<dyn std::error::Error>> {
  let input = include_str!("./input");
  let bites = input
    .lines()
    .map(|s| i32::from_str_radix(s, 2).unwrap())
    .collect::<Vec<_>>();

  let oxy = recurse(WIDTH as i32 - 1, bites.clone(), usize::ge);
  let co2 = recurse(WIDTH as i32 - 1, bites, usize::lt);

  Ok(oxy * co2)
}

fn main() {
  println!("{}", day3a().unwrap())
}
