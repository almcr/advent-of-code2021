/// 1 => 2
/// 7 => 3       
/// 4 => 4        
/// 2 => 5
/// 3 => 5
/// 5 => 5       
/// 6 => 6       
/// 9 => 6       
/// 8 => 7       
/// dddd
// e    a
// e    a
//  ffff
// g    b
// g    b
//  cccc

fn day8a() -> Result<i32, Box<dyn std::error::Error>> {
  Ok(
    include_str!("./input")
      .lines()
      .map(|l| {
        l.split_once('|')
          .unwrap()
          .1
          .split(' ')
          .map(|d| match d.len() {
            2 | 3 | 4 | 7 => 1,
            _ => 0,
          })
          .sum::<i32>()
      })
      .sum(),
  )
}

fn main() {
  println!("{}", day8a().unwrap())
}

///
/// 3 => 6
///
fn mask(num: &str) -> usize {
  num.chars().map(|c| c as usize).sum::<usize>()
}

#[test]
fn test() {
  println!("0 {}", mask("fdcagb"));
  println!("1 {}", mask("ab"));
  println!("2 {}", mask("gcdfa"));
  println!("3 {}", mask("fcadb"));
  println!("4 {}", mask("eafb"));
  println!("5 {}", mask("cdfeb"));
  println!("6 {}", mask("cdfgeb"));
  println!("7 {}", mask("dab"));
  println!("8 {}", mask("acedgfb"));
  println!("9 {}", mask("cefabd"));
}
