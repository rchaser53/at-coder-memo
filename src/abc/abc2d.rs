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
 
#[fastout]
fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }
  let mut max = 1;
  let limit = 1 << n;
  let mut memo = vec![0;n];
  for i in 0..n {
    memo[i] = 1 << i;
  }

  for i in 0..m {
    let (a, b) = vals[i];
    memo[a] |= (1 << b);
    memo[b] |= (1 << a);
  }

  for i in 1..limit {
    let mut temp = 0;
    for ii in 0..n {
      if i >> ii & 1 == 1 {
        if memo[ii] & i == i {
          temp += 1;
        }
      }
    }
    max = std::cmp::max(max, temp);
  }
  println!("{}", max);
}