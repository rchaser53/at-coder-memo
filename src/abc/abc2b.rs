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
    w: Chars,
  }
  let mut result = vec![];
  let mut set = hashset!{ 'a', 'i', 'u', 'e', 'o' };
  for c in w {
    if !set.contains(&c) {
      result.push(c);
    }
  }
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
  
}