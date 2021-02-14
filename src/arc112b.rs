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

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    b: i128,
    c: i128
  }
  
  println!("{}",
    if b == 0 {
      c/2 + (c-1)/2 + 1
    } else if 0 <= b {
      let v1 = std::cmp::min(b, (c-1)/2 + 1);
      let v2 = std::cmp::min(b+1, c/2+1);
      (c-1)/2 + v1 + v2 + (c-2)/2
    } else {
      let v1 = std::cmp::min(-b-1, (c-2)/2);
      let v2 = std::cmp::min(-b+1, (c-1)/2 + 1);
      c/2 + 1 + v1 + v2 + (c-1)/2
    }
  );
}

