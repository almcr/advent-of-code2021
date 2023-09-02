fn day7a() -> Result<i32, Box<dyn std::error::Error>> {
  let mut pos: Vec<i32> = include_str!("./input")
    .split(",")
    .map(|c| c.parse().unwrap())
    .collect();
  pos.sort();

  let pivot = pos[pos.len() / 2];
  let cost = pos.iter().map(|h| i32::abs(h - pivot)).sum();

  Ok(cost)
}

fn main() {
  println!("{}", day7a().unwrap())
}
