const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn main() {
  let grpah = include_str!("input")
    .lines()
    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
    .collect::<Vec<Vec<_>>>();

  println!("{}", bfs(&grpah));
}

fn bfs(graph: &Vec<Vec<u32>>) -> f32 {
  let mut dist = vec![f32::MAX - 100.; graph.len() * graph[0].len()];
  dist[0] = 0.;
  let mut visited = vec![false; graph.len() * graph[0].len()];
  let cols = graph[0].len() as i32;
  visited[0] = true;

  for i in 0..graph.len() {
    for j in 0..graph[i].len() {
      if dist[(i * cols as usize) + j] > *dist.last().unwrap() {
        continue;
      }

      NEIGHBORS.iter().for_each(|(ii, jj)| {
        let ii = i as i32 + *ii;
        let jj = j as i32 + *jj;

        if let Some(n) = graph.get(ii as usize).and_then(|row| row.get(jj as usize)) {
          let nl = dist[(i * cols as usize) + j] + (*n as f32);
          let idx = ((ii * cols) + jj) as usize;

          if !visited[idx] && nl < dist[idx] {
            dist[idx] = nl;
          }
        }
      });

      visited[(i * cols as usize) + j] = true;
    }
  }
  *dist.last().unwrap()
}
