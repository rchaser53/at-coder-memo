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
    n: usize
  }
  
  if n == 1 {
    println!("Deficient");
    return
  }
  
  let mut a = 2;
  let mut set = HashSet::new();
  while a * a <= n {
    if n % a == 0 {
      set.insert(a);
      set.insert(n/a);
    }
    a += 1;
  }
  set.insert(1);
  
  let mut total = 0;
  for v in set {
    total += v;
  }
  
  if total == n {
    println!("Perfect");
  } else if total < n {
    println!("Deficient");
  } else {
    println!("Abundant");
  }
  
}