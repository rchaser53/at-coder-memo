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
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1,isize);m]
  }
  
  let mut neighbors = vec![vec![];n];
  for (from, to, v) in vals {
    neighbors[from].push((to, -1 * v));
  }
  
  let inf = 1<<60;
  let mut memo = vec![inf;n];
  let mut negative = false;
  memo[0] = 0;
  
  for i in 0..=n {
    for ii in 0..n {
      for e in &neighbors[ii] {
        if memo[ii] + e.1 < memo[e.0] {
          memo[e.0] = memo[ii] + e.1;
          if e.0 == n-1 && i == n {
            negative = true;
          } 
        }
      }
    }
  }
  if negative {
    println!("inf");
  } else {
    println!("{}", -1 * memo[n-1]);
  }
}