/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
    if score[u] < w_u {
      continue
    }
    
    for &(v, w_v) in graph[u].iter() {
      let w = w_u + w_v;
      if w < score[v] {
        score[v] = w;
        pq.push(std::cmp::Reverse((w, v)));
      }
    }
  }
  score
}
fn main() {
  input! {
    n:usize,
    edges: [(Usize1,Usize1);n-1]
  }

  let mut g = vec![vec![];n];
  for (a, b) in edges {
    g[a].push((b, 1));
    g[b].push((a, 1));
  }

  let inf = 1_000_000_000;
  let base = dijkstra(n, inf, &g, 0);
  let mut max = 0;
  let mut start = 0;
  for i in 0..n {
    if max < base[i] {
      start = i;
      max = base[i];
    }
  }

  let base = dijkstra(n, inf, &g, start);
  let mut max = 0;
  let mut goal = 0;
  for i in 0..n {
    if max < base[i] {
      goal = i;
      max = base[i];
    }
  }

  println!("{} {}", start+1, goal+1);
}