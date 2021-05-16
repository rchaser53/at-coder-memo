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
    n:usize,
    vals:[(usize,usize);n]
  }
  
  let mut map = HashMap::new();
  for (from, to) in vals {
    *map.entry(from).or_insert(0) += 1isize;
    *map.entry(to+1).or_insert(0) -= 1isize;
  }
  
  let limit = 1_000_010;
  let mut memo = vec![0;limit];
  if let Some(v) = map.get(&0) {
    memo[0] += *v;
  }
  
  for i in 1..limit {
    if let Some(v) = map.get(&i) {
      memo[i] += *v;
    }
    memo[i] += memo[i-1];
  }
  
  let mut max = 0;
  for i in 0..limit {
    max = std::cmp::max(max, memo[i]);
  }
  println!("{}", max);
}