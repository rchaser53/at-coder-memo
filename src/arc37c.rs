#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use superslice::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n:usize,
    k:Usize1,
    mut a:[usize;n],
    mut b:[usize;n]
  }
  a.sort();
  b.sort();
  
  let mut min = 1;
  let mut max = 1_000_000_000 * 1_000_000_000 * 2;
  while min + 1 < max {
    let mid = (min+max) / 2;
    let mut temp = 0;
    for i in 0..n {
      let v = mid / a[i];
      temp += b.upper_bound(&v);
    }
    
    if k < temp {
      max = mid;
    } else {
      min = mid;
    }
  }
  println!("{}", max);
}