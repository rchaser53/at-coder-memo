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
    s: Chars
  }
  
  let s_len = s.len();
  let mut map = HashMap::new();
  for c in s {
    *map.entry(c).or_insert(0) += 1;
  }
  
  let mut odd_num = 0;
  let mut should_remove = vec![];
  for (key, val) in map.iter_mut() {
    if *val % 2 == 1 {
      odd_num += 1;
      if *val == 1 {
        should_remove.push(key.clone());
      } else {
        *val -= 1;
      }
    }
  }
  
  for key in should_remove {
    map.remove(&key);
  }
  
  if odd_num <= 1 {
    println!("{}", s_len);
    return
  }
  
  let mut total = 0;
  for (_, val) in map {
    total += val / 2;
  }
  
  if total == 0 {
    println!("1");
  } else {
    println!("{}", 2 * (total / odd_num) + 1);
  }
}