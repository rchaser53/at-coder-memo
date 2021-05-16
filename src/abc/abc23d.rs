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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    vals: [(usize,usize);n]
  }
  
  let mut min = 0;
  let mut max = 2 * n * 1_000_000_000;
  while min + 1 < max {
    let mid = (min+max) / 2;

    let mut ok = true;
    let mut memo = vec![0;n];
    for i in 0..n {
      let (h, s) = vals[i];
      if mid < h {
        ok = false;
      } else {
        memo[i] = (mid - h) / s;
      }
    }
    memo.sort();
    for i in 0..n {
      if memo[i] < i {
        ok = false;
      }
    }    

    if ok {
      max = mid;
    } else {
      min = mid;
    }
  }
  
  println!("{}", max);
}