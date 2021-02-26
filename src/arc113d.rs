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
 
const MOD:usize = 998244353;
 
fn repeat_square(n:usize, p:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2).pow(2) % MOD 
  } else {
    n * repeat_square(n, p-1) % MOD
  }
}

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    k: usize
  }
  
  if n == 1 {
    println!("{}", repeat_square(k, m));
    return
  } else if m == 1 {
    println!("{}", repeat_square(k, n));
    return
  }
  
  let mut result = 0;
  for i in 1..=k {
    result = (result +
      (repeat_square(i, n) + MOD - repeat_square(i-1, n)) * 
      repeat_square(k-i+1, m)
    ) % MOD;
  }
  println!("{}", result);
}