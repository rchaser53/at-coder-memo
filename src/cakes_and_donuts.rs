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
    n: isize
  }
  
  for i in 0..30 {
    for ii in 0..20 {
      let v = i * 4 + ii * 7;
      if n < v { break }
      if n == v {
        println!("Yes");
        return
      }
    }
  }

  println!("No");
}