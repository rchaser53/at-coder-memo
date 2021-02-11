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

fn main() {
  input! {
    n: usize,
    vals: [(isize, isize);n]
  }
  
  let mut count = 0;
  for i in 0..n {
    let (x, y) = vals[i];
    for ii in i+1..n {
      let (x1, y1) = (vals[ii].0 - x, vals[ii].1 - y);
      for iii in ii+1..n {
        let (x2, y2) = (vals[iii].0 - x, vals[iii].1 - y);
        let v = x1*y2 - x2*y1;
        if v % 2 == 0 && v != 0 {
          count += 1;
        }
      }
    }
  }
  println!("{}", count); 
}