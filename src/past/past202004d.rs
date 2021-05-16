#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    s: Chars
  }
  let mut set = HashSet::new();
  for i in 0..s.len() {
    let mut stack = vec![];
    for ii in 0..3 {
      if s.len() <= i + ii { break }
      stack.push(s[i + ii]);
      set.insert(stack.clone());
    }
  }
  let mut result = HashSet::new();
  
  for v in set {
    let len = v.len();
    let limit = 1 << len;
    for ii in 0..limit {
      let mut cloned = v.clone();
      for iii in 0..len {
        if ii >> iii & 1 == 1 {
          cloned[iii] = '.';
        }
      }
      result.insert(cloned);
    }
  }

  println!("{}", result.len());
}