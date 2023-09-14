use std::collections::HashMap;

pub fn main() {
  let (base, rules) = include_str!("input").split_once("\n\n").unwrap();
  let base = base.chars().collect::<Vec<_>>();
  let rules: HashMap<(char, char), char> = rules
    .lines()
    .map(|l| l.split_once(" -> ").unwrap())
    .map(|(p, t)| {
      (
        (p.as_bytes()[0] as char, p.as_bytes()[1] as char),
        t.as_bytes()[0] as char,
      )
    })
    .collect();

  let mut letter_count: HashMap<_, _> = base.iter().map(|c| (*c, 1)).collect();
  let mut pairs_count = base.windows(2).fold(HashMap::new(), |mut m, p| {
    m.entry((p[0], p[1]))
      .and_modify(|v| *v += 1_usize)
      .or_insert(1);
    m
  });

  for _ in 0..40 {
    let mut pairs_count_next = HashMap::new();
    pairs_count.keys().for_each(|p| {
      let p_count = *pairs_count.get(p).unwrap_or(&1);
      let new_char = *rules.get(p).unwrap();

      letter_count
        .entry(new_char)
        .and_modify(|v| *v += p_count)
        .or_insert(p_count);

      for p in [(p.0, new_char), (new_char, p.1)] {
        pairs_count_next
          .entry(p)
          .and_modify(|v| *v += p_count)
          .or_insert(p_count);
      }
    });
    pairs_count = pairs_count_next;
  }

  let min = letter_count.values().min().unwrap();
  let max = letter_count.values().max().unwrap();
  println!("{}", max - min + 1);
}
