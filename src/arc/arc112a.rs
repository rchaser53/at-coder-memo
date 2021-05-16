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
    n:usize,
    vals:[(usize, usize);n]
  }  
  for (l, r) in vals {
    if r < 2 * l {
      println!("0");
    } else {
      let num = r - 2 * l + 1;
      println!("{}", (1+num)*num/2);
    }
  }
}