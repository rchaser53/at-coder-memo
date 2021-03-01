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
const MAX: usize = 1000;

fn main() {
  input!{
    mut n: usize
  }
  
  if n == 1 {
    println!("Not Prime");
    return
  }
  
  let mut set = HashSet::new();
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      set.insert(i);
    }
    i += 1;
  }
  
  if set.is_empty() {
    println!("Prime");
    return
  } else if n % 5 == 0 || n % 2 == 0 {
    println!("Not Prime");
    return
  }
  
  let mut count = 0;
  while 0 < n {
    count += n % 10;
    n /= 10;
  }
  
  if count % 3 != 0 {
    println!("Prime");
  } else {
    println!("Not Prime");
  }
}