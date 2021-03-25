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
    s:Chars
  }
  
  let mi = s.len()-1;
  for i in 0..s.len()/2 {
    if s[i] != s[mi-i]
      && s[i] != '*'
      && s[mi-i] != '*' {
      println!("NO");
      return      
    }
  }
  
  println!("YES");
}