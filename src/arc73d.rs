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
use superslice::Ext;

const MOD:usize = 998244353;
const MAX:usize = 200_000;

fn main() {
  input!{
    n:usize,
    w:usize,
    vals:[(usize,usize);n]
  }
  
  let mut map = HashMap::new();
  map.insert(0, 0);
  
  for i in 0..n {
    let (cw, cv) = vals[i];
    let mut new_map = map.clone();
    for (key, val) in &map {
      let nw = key + cw;
      if nw <= w {
        let nv = val + cv;
        let nv = std::cmp::max(
          *map.get(&nw).unwrap_or(&nv),
          nv
        );
        
        let nnv = *new_map.get(&nw).unwrap_or(&nv);
        new_map.insert(nw, std::cmp::max(nnv, nv));
      }
    }
    map = new_map;
  }
  let mut max = 0;
  for (_, val) in map {
    max = std::cmp::max(max, val);
  }
  println!("{}", max);
}