/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

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
    m:usize,
    vals:[usize;n],
    xyz:[(Usize1,Usize1,Usize1);m]
  }

  let limit = 1 << n;
  let mut g = vec![vec![];limit];

  for (x,y,z) in xyz {
    for i in 0..limit {
      let mut ni = i;
      ni ^= 1 << x;
      ni ^= 1 << y;
      ni ^= 1 << z;
      g[i].push((ni,1));
    }
  }

  let mut temp = 0;
  for i in 0..n {
    if vals[i] == 1 {
      temp |= 1 << i;
    }
  }
  let inf = 1_000_000_000;
  let result = dijkstra(limit, inf, &g, temp);

  if result[limit-1] == inf {
    println!("-1");
  } else {
    println!("{}", result[limit-1]);
  }
}