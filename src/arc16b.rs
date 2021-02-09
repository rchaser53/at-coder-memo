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
    rows:[Chars;n]
  }
  
  let mut set = HashSet::new();
  let mut count = 0;
  for i in 0..n {
    for ii in 0..9 {
      let mut i = i;
      if set.contains(&(i, ii)) || rows[i][ii] == '.' { continue }
      count += 1;
      set.insert((i, ii));
      while i < n && rows[i][ii] == 'o' {
        set.insert((i, ii));
        i += 1;
      }
    }
  }
  println!("{}", count);
}