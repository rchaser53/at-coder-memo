/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

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
 
fn adjacency_list(
  n: usize,
  uvw: &Vec<(usize, usize, usize)>
) -> Vec<Vec<(usize, usize)>> {
  let mut e = vec![vec![]; n];
  for &(u, v, w) in uvw.iter() {
    e[u].push((v, w));
    e[v].push((u, w));
  }
  e
}

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    t:usize,
    vals:[usize;n],
    edges:[(Usize1,Usize1,usize);m]
  }

  let mut normal = vec![vec![];n];
  let mut back = vec![vec![];n];

  for &(u, v, w) in &edges {
    if v == 0 { continue }
    normal[u].push((v, w));
  }
  for &(u, v, w) in &edges {
    if u == 0 { continue }
    back[v].push((u, w));
  }

  let inf = usize::max_value();
  let first = dijkstra(n, inf, &normal, 0);
  let goal = dijkstra(n, inf, &back, 0);

  let mut max = t * vals[0];
  for i in 1..n {
    if first[i] == inf || goal[i] == inf { continue }
 
    let sum = first[i] + goal[i];
    if t <= sum { continue }
    max = std::cmp::max(max, (t - sum) * vals[i]);
  }
  println!("{}", max);
}