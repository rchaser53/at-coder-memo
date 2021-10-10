use proconio::input;
use proconio::marker::*;
use std::collections::*;

// 経路復元 dijkstra
fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
  prev: &mut Vec<usize>,
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((cv, ci))) = pq.pop() {
    if score[ci] < cv {
      continue
    }
    
    for &(av, ni) in &graph[ci] {
      let nv = av + cv;
      if nv < score[ni] {
        score[ni] = nv;
        prev[ni] = ci;
        pq.push(std::cmp::Reverse((nv, ni)));
      }
    }
  }
  score
}

fn main() {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }

  
  let mut g = vec![vec![];n];
  for (a, b) in edges {
    g[a].push((1, b));
    g[b].push((1, a));
  }
  let mut routes = vec![0;n];
  dijkstra(n, 1_000_000_000, &g, 0, &mut routes);

  println!("Yes");
  for i in 1..n {
    println!("{}", routes[i] + 1);
  }
}