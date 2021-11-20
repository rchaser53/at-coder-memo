/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
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

// ベルマンフォード
fn bellman_ford(
  g: &Vec<Vec<(usize, isize)>>,
  p: isize,
  inf:isize,
) -> Option<Vec<isize>> {
  let n = g.len();
  let mut memo = vec![inf;n];
  memo[0] = 0;

  for i in 0..n {
    let mut update = false;
    for ci in 0..n {
      if memo[ci] == inf { continue }
      for &(ni, v) in &g[ci] {
        let nv = memo[ci] + v - p;
        if memo[ni] < nv {
          memo[ni] = nv;
          update = true;
        }
      }
    }

    if !update { break }
    if i == n-1 && update {
      
      return None
    }
  }
  Some(memo)
}

fn main() {
  input! {
    n:usize,
    m:usize,
    p:isize,
    edges:[(Usize1,Usize1,isize);m]
  }

  let mut fg = vec![vec![];n];
  for &(f, t, _) in &edges {
    fg[t].push((f, 1));
  }
  let inf = 1_000_000_000;
  let temp = dijkstra(n, inf, &fg, n-1);

  let mut g = vec![vec![];n];
  for (f, t, v) in edges {
    if temp[f] == inf || temp[t] == inf { continue }
    g[f].push((t, v));
  }

  if let Some(memo) = bellman_ford(&g, p, -1_000_000_000_000isize) {
    println!("{}", std::cmp::max(memo[n-1], 0));
  } else {
    println!("-1");
  }
}