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
    mut s: Chars
  }
  
  let n = s.len();
  let mut count = 0;
  s.reverse();
  
  let mut i = 0;
  let mut map = HashMap::new();
  while i < n-2 {
    if s[i] != s[i+1] && s[i+1] == s[i+2] {
      count += i+1;
      if let Some(v) = map.get(&s[i+1]) {
        count -= v;
      }
      map = HashMap::new();
      map.insert(s[i+1], i+1);
    } else {
      *map.entry(s[i]).or_insert(0) += 1;
    }
    i += 1;
  }
  println!("{}", count);
}