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
const MAX:usize = 1000;

fn culc(x:usize) -> bool {
  let mut i = 2;
  while i * i <= x {
    if x % i == 0 {
      return false
    } else {
      i += 1;
    }
  }
  true
}

fn main() {
  input!{
    n:usize,
    mut vals:[usize;n]
  }  
  
  let mut nums = vec![];
  for i in 2..=50 {
    if culc(i) {
      nums.push(i);
    }
  }
  let limit = 1 << nums.len();
  let mut min = 1_000_000_000_000_000_000usize;
  for i in 0..limit {
    let mut set = HashSet::new();
    for ii in 0..nums.len() {
      if i >> ii & 1 == 1 {
        for iii in 0..n {
          if vals[iii] % nums[ii] == 0 {
            set.insert(iii);
          }
        }
      }
    }
    if set.len() == n {
      let mut v = 1;
      for ii in 0..nums.len() {
        if i >> ii & 1 == 1 {
          v *= nums[ii];
        }
      }

      min = std::cmp::min(min, v);
    }
  }
  
  println!("{}", min);
}