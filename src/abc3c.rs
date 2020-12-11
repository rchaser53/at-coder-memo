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
    k: usize,
    mut vals: [usize;n]
  }
  
  vals.sort();
  let mut vals: Vec<f64> = vals.into_iter().map(|v| v as f64).collect();
  
  let mut v = 0f64;
  for i in n-k..n {
    v = (v + vals[i]) / 2f64;
  }
  
  println!("{}", v);
}