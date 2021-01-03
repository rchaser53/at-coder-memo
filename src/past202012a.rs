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

fn main() {
  input!{
    s: Chars,
  }
  
  for i in 0..3 {
    if s[i] == s[i+1] && s[i+1] == s[i+2] {
      println!("{}", s[i]);
      return
    }
  }
  println!("draw");
}