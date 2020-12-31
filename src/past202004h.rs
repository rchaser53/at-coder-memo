#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    rows: [Chars;n]
  }
  
  let mut dp = vec![vec![1_000_000_000;m];n];
  let mut points = vec![vec![];11];
  for y in 0..n {
    for x in 0..m {
      let ti = rows[y][x];
      if let Some(ti) = ti.to_digit(10) {
        points[ti as usize].push((x, y));
      } else if ti == 'S' {
        points[0].push((x, y));
      } else if ti == 'G' {
        points[10].push((x, y));
      }
    }
  }
  
  let (sx, sy) = points[0][0];
  dp[sy][sx] = 0;
    
  for (prev, now) in points.iter().tuple_windows() {
    for &now in now {
      for &prev in prev {
        let v = (now.0 as isize - prev.0 as isize).abs()
          + (now.1 as isize - prev.1 as isize).abs();
        dp[now.1][now.0] = std::cmp::min(dp[now.1][now.0], dp[prev.1][prev.0] + v);
      }
    }
  }
  
  let (gx, gy) = points[10][0];
  if dp[gy][gx] == 1_000_000_000 {
    println!("-1");
  } else {
    println!("{}", dp[gy][gx]);
  }
}


