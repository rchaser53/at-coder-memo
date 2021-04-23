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

fn gcv(a: usize, b: usize) -> usize { 
  if b == 0 {
    a
  } else {
    gcv(b, a % b)
  }
}

fn main() {
  input!{
    vals:[usize;3]
  }
  let gv = gcv(vals[0], vals[1]);
  let gv = gcv(vals[2], gv);
  
  let mut count = 0;
  for v in vals {
    count += (v / gv) - 1;
  }
  println!("{}", count);
}