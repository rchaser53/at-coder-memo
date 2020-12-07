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
    m: usize,
    vals: [(Usize1, Usize1, usize);m]
  }
  
  let g = UnGraph::<usize, usize, usize>::from_edges(&vals);  

  let mut min = 1_000_000_000;
  for i in 0..n {
    let mut max = 0;
    let res = dijkstra(&g, i.into(), None, |e| *e.weight());
    for ii in 0..n {
      if i == ii { continue }
      if let Some(v) = res.get(&NodeIndex::new(ii)) {
        max = std::cmp::max(max, *v);
      }
    }
    min = std::cmp::min(min, max);
  }
  
  println!("{}", min);
}