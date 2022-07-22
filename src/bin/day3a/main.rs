const TEST_STR: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

const LINE_COUNT: usize = 1000;
const WIDTH: usize = 12;

fn day3a() -> Result<u32, Box<dyn std::error::Error>> {
  let mut ones = [0; WIDTH];
  for bites in include_str!("./input").lines() {
  // for bites in TEST_STR.lines() {
    for (i, b) in bites.chars().enumerate() {
      if b == '1' {
        ones[i] += 1;
      }
    }
  }

  let gamma = ones
  .into_iter()
  .rev()
  .enumerate()
  .map(|(i, c)| ((c >= LINE_COUNT / 2) as u32) << i)
  .sum::<u32>();

  // Flipping gamma as it is will also flip (set) the bites above the
  // most significant one. To avoid this, we need a way to re-flip
  // all those, by masking them with zeros.
  // If we shift 1 after MSB positionand then substract 1
  // it will set all bites after MSB.
  // [Example]
  // 16 === 0b10000
  // 16 - 1 = 15 === 0b01111

  Ok(gamma * (!gamma & ((1 << WIDTH) - 1)))
}

fn main() {
  println!("{}", day3a().unwrap())
}
