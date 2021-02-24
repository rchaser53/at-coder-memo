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
    l: usize,
    r: usize,
    ls: [usize;l],
    mut rs: [usize;r]
  }
  
  let mut count = 0;
  for v in ls {
    for i in 0..rs.len() {
      if v == rs[i] {
        count += 1;
        rs.remove(i);
        break
      }
    }
  }
  println!("{}", count);
}