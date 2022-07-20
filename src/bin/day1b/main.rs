use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Read the input file into buffer
  let file = File::open("src/bin/day1/input")?;
  let reader = BufReader::new(file);

  // Iterate over the buffer and sum the numbers larger than previous ones
  let lines = reader.lines();

  let res = lines.map(|l| l.unwrap().parse::<u16>().unwrap())
  .tuple_windows()
  .filter(|(a, _, _, b)| a < b)
  .count();

  println!("{}", res);

  Ok(())
}
