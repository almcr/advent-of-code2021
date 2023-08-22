/// dddd
// e    a
// e    a
//  ffff
// g    b
// g    b
//  cccc

fn hash(num: &str) -> usize {
  num.chars().map(|c| c as usize).sum::<usize>()
}

fn day8b() -> Result<i32, Box<dyn std::error::Error>> {
  Ok(
    include_str!("./test")
      .lines()
      .map(|l| {
        l.split_once(" | ")
          .unwrap()
          .1
          .split(' ')
          .map(|d| match d.len() {
            2 => '1',
            3 => '7',
            4 => '4',
            7 => '8',
            _ => match hash(d) {
              598 => '0',
              501 => '2',
              496 => '3',
              500 => '5',
              603 => '6',
              597 => '9',
              _ => {
                println!("{}", d);
                '0'
              }
            },
          })
          .collect::<String>()
          .parse::<i32>()
          .unwrap()
      })
      .sum(),
  )
}

fn main() {
  println!("{}", day8b().unwrap())
}

#[test]
fn test23() {
  println!("0 {}", hash("fdcagb"));
  println!("1 {}", hash("ab"));
  println!("2 {}", hash("gcdfa"));
  println!("3 {}", hash("fcadb"));
  println!("4 {}", hash("eafb"));
  println!("5 {}", hash("cdfeb"));
  println!("6 {}", hash("cdfgeb"));
  println!("7 {}", hash("dab"));
  println!("8 {}", hash("acedgfb"));
  println!("9 {}", hash("cefabd"));
}
