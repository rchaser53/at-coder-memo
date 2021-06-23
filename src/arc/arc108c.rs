/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    edges:[(usize,usize,usize);m]
  }

  let mut neighbors = vec![vec![];n];
  for &(from, to, label) in &edges {
    neighbors[from-1].push((to-1, label));
    neighbors[to-1].push((from-1, label));
  }

  let mut result = vec![0;n];
  let mut seen = vec![false;n];
  result[0] = 1;
  
  let mut que = VecDeque::new();
  que.push_back(0);

  while let Some(ci) = que.pop_front() {
    seen[ci] = true;
    let now = result[ci];
    for &(w, c) in &neighbors[ci] {
      if seen[w] { continue }
      result[w] = if now == c {
        ((c+1) % n).max(1)
      } else {
        c
      };
      que.push_back(w);
    }
  }

  for v in result {
    println!("{}", v);
  }
}