/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    s:Chars,
    t:Chars,
  }

  let mut result = format!("");
  for i in 0..n {
    result = format!("{}{}{}", result, s[i], t[i]);
  }
  println!("{}", result);
}
