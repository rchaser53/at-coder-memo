/* OUTPUT FILE */

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1, Usize1);m]
  }

  let mut g = vec![vec![];n];
  for (l, r) in edges {
    g[l].push(r);
    g[r].push(l);
  }

  let inf = 1_000_000_000_000usize;
  let mut memo = vec![inf;n];
  let mut counts = vec![inf;n];
  counts[0] = 1;
  memo[0] = 0;

  let mut stack = vec![(0,0)];
  // que.push_back((0,0));
  while !stack.is_empty() {
    let mut new_stack = vec![];

    while let Some((ci, cv)) = stack.pop() {
      for &ni in &g[ci] {
        let nv = (cv + 1) % MOD;
        if nv < memo[ni] {
          memo[ni] = nv;
          counts[ni] = counts[ci];
          new_stack.push((ni, nv));
        } else if memo[ni] == nv {
          counts[ni] += counts[ci];
          counts[ni] %= MOD;
        }
      }
    }
    stack = new_stack;
  }

  if memo[n-1] == inf {
    println!("0");
  } else {
    println!("{}", counts[n-1]);
  }
}
