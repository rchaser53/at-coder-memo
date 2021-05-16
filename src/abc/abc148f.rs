#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};

fn main() {
  input!{
    n:usize,
    u:Usize1,
    v:Usize1,
    vals:[(Usize1,Usize1);n-1]
  }
  
  let mut base = vec![];
  for (from, to) in vals {
    base.push((from, to, 1usize));
  }
  let g = UnGraph::<usize, usize, usize>::from_edges(&base);
  let res_u = dijkstra(&g, u.into(), Some(n.into()), |e| *e.weight());
  let res_v = dijkstra(&g, v.into(), Some(n.into()), |e| *e.weight());
  
  let mut result = 0;
  for i in 0..n {
    let uv = *res_u.get(&NodeIndex::new(i)).unwrap();
    let vv = *res_v.get(&NodeIndex::new(i)).unwrap();
    if uv < vv {
      result = std::cmp::max(result, vv);
    }
  }
  
  println!("{}", result - 1);
}