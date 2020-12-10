#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    rows: [[isize;n];n],
    q: usize,
    vals: [usize;q]
  }
  
  let mut culc = vec![vec![0;n+1];n+1];
  for y in 0..n {
    for x in 0..n {
      culc[y+1][x+1] = culc[y+1][x] + culc[y][x+1] - culc[y][x] + rows[y][x];
    }
  }

  let mut memo = vec![0;n*n+1];
  for x1 in 0..n {
    for x2 in x1+1..=n {
      for y1 in 0..n {
        for y2 in y1+1..=n {
          let v = (x2-x1) * (y2-y1);
          memo[v] = std::cmp::max(
            memo[v],
            culc[y2][x2] - culc[y1][x2] - culc[y2][x1] + culc[y1][x1]
          );
        }
      }
    }
  }
  
  for v in 1..=n*n {
    memo[v] = std::cmp::max(memo[v], memo[v-1]);
  }
  
  for v in vals {
    println!("{}", memo[v]);
  }
}