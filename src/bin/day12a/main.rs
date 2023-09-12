use std::collections::BTreeMap;

use itertools::Itertools;

fn traverse(
  graph: &BTreeMap<u8, Vec<u8>>,
  start: &u8,
  visits: &mut BTreeMap<u8, i32>,
) -> i32 {
  if start == &1 {
    return 1;
  }
  let mut c = 0;
  if let Some(neighbors) = graph.get(start) {
    for nei in neighbors.iter().rev() {
      println!("{:?}", visits.get(nei));
      match visits.get(nei) {
        Some(-1) | Some(1) => {
          if let Some(1) = visits.get(nei) {
            visits.entry(*nei).and_modify(|v| *v = 0);
          }
          c += traverse(graph, &nei, visits);
          if let Some(0) = visits.get(nei) {
            visits.entry(*nei).and_modify(|v| *v = 1);
          }
        }
        _ => (),
      };
    }
    return c;
  }
  0
}

fn main() {
  let mut node_ids = BTreeMap::<&str, u8>::from([("start", 0), ("end", 1)]);
  let mut adjacency = BTreeMap::<u8, Vec<u8>>::new();
  let mut visits = BTreeMap::<u8, i32>::from([(0, 0), (1, 1)]);
  for l in include_str!("input").lines() {
    let (x, y) = l.split('-').next_tuple().unwrap();
    let mut idx = |n| {
      let i = node_ids.len() as u8;
      let x = node_ids.entry(n).or_insert(i);

      visits
        .entry(*x)
        .or_insert((n.as_bytes()[0] <= b'Z').then(|| -1).unwrap_or(1));
      *x
    };

    let mut branch = |a, b| {
      println!("{:?}", (a, b));
      let (a, b) = (idx(a), idx(b));
      println!("{:?}", (a, b));
      adjacency
        .entry(a)
        .and_modify(|n| n.push(b))
        .or_insert(vec![b]);
    };

    branch(x, y);
    branch(y, x);
  }

  let x = traverse(&adjacency, &0, &mut visits);
  println!("{}", x);
}
