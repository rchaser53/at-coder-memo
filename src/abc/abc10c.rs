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
    xa: f64,
    ya: f64,
    xb: f64,
    yb: f64,
    t: f64,
    v: f64,
    n: usize,
    vals: [(f64,f64);n]
  }
  
  let limit = t * v;
  for (x, y) in vals {
    let before = ((x - xa).powi(2) + (y - ya).powi(2)).sqrt();
    let after = ((x - xb).powi(2) + (y - yb).powi(2)).sqrt();
    
    if before + after <= limit {
      println!("YES");
      return
    }
  }
  println!("NO");
}