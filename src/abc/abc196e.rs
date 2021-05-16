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
use superslice::*;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
  input!{
    n:usize,
    vals:[(isize,usize);n],
    q:usize,
    queries:[isize;q]
  }
  
  let mut down = -1_000_000_001;
  let mut up = 1_000_000_001;
  let mut change = 0;
  for (v, t) in vals {
    if t == 1 {
      change += v;
      down += v;
      up += v;
    } else if t == 2 {
      down = std::cmp::max(v, down);
      up = std::cmp::max(v, up);
    } else {
      down = std::cmp::min(v, down);
      up = std::cmp::min(v, up);
    }
  }
  for v in queries {
    println!("{}", std::cmp::min(std::cmp::max(down, v + change), up));
  }
}