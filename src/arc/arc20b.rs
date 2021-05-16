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
  input! {
    n: usize,
    c: usize,
    vals: [usize;n]
  }
  
  let mut min = 1_000_000_000;
  for i in 1..=10 {
    for ii in 1..=10 {
      if i == ii { continue }
      let mut temp = 0;
      for iii in 0..n {
        if iii % 2 == 0 {
          if vals[iii] != i {
            temp += c;
          }
        } else {
          if vals[iii] != ii {
            temp += c;
          }
        }
      }
      min = std::cmp::min(min, temp);
    }
  }
  println!("{}", min);
}