const TEST_STR : &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (mut h, mut v) = (0, 0);

  for line in include_str!("./input").lines() {
    // for line in TEST_STR.lines() {
    let (command, value) = line.split_once(' ').unwrap();
    let value: i32 = value.parse()?;
    match command {
      "forward" => h += value,
      "up" => v -= value,
      "down" => v += value,
      &_ => unreachable!(),
    }
  }

  println!("{}", h * v);

  Ok(())
}