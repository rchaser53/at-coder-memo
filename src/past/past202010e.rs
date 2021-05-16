#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut s: Chars
  }
  
  let strs = s.clone()
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>();
  let mut reverse = strs.clone();
  reverse.reverse();
  let reverse = reverse.into_iter().collect::<String>();
  let normal = strs.into_iter().collect::<String>();
  
  if n <= 2 {
    println!("None");
    return
  }
  
  let mut heap = Heap::new(&mut s);
  while let Some(elt) = heap.next_permutation() {
    let v = elt.into_iter()
      .map(|v| v.to_string())
      .collect::<String>();
    if v != normal && v != reverse {
      println!("{}", v);
      return
    }
  }
  println!("None");
}