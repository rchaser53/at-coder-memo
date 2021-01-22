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
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let limit = 3*10usize.pow(5)+1;
  let mut memo = vec![0;limit];
  for v in vals {
    memo[v] += 1;
  }
  let mut result = vec![0;k];
  for i in 0..k {
    let mut ii = 0;
    while ii < limit {
      if memo[ii] == 0 {
        break
      }
      memo[ii] -=1;
      ii += 1;
    }
    result[i] = ii;
  }
  println!("{}", result.into_iter().sum::<usize>());
}