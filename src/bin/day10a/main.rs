fn score(c: char) -> i32 {
  match c {
    '}' => 1197,
    ')' => 3,
    ']' => 57,
    '>' => 25137,
    _ => 0,
  }
}

fn find_illegal(line: &str) -> Option<char> {
  let mut opening = Vec::with_capacity(64);

  for c in line.chars() {
    match c {
      '(' | '{' | '<' | '[' => opening.push(c),
      ')' => {
        if Some('(') != opening.pop() {
          return Some(')');
        }
      }
      c => {
        if Some((c as u8 - 2) as char) != opening.pop() {
          return Some(c);
        }
      }
    }
  }
  None
}

pub fn main() {
  let score: i32 = include_str!("input")
    .lines()
    .map(|l| score(find_illegal(l).unwrap_or(' ')))
    .sum();

  println!("{}", score);
}

#[test]
fn foo() {
  let c = '}';
  println!("{}", (c as u8 - 2) as char);
}
