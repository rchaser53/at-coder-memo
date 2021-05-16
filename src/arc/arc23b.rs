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
    r: usize,
    c: usize,
    d: usize,
    rows: [[usize;c];r]
  }
  
  let is_even = d % 2 == 1;
  let mut max = 0;
  for i in 0..r {
    for ii in 0..c {
      if d < i + ii { continue }
      if is_even {
        if (i+ii) % 2 == 1 {
          max = std::cmp::max(max, rows[i][ii]);
        }
      } else {
        if (i+ii) % 2 == 0 {
          max = std::cmp::max(max, rows[i][ii]);
        }
      }
    }
  }
  println!("{}", max);
}