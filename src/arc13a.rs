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
    base_box: [usize;3],
    mut item: [usize;3]
  }
  
  let mut max = 0;
  let mut sorted = base_box.clone();
  sorted.sort();
  item.sort();
  loop {
    let mut copied = item.clone();
    loop {
      let mut count = 1;
      for i in 0..3 {
        count *= sorted[i] / copied[i];
      }
      max = std::cmp::max(max, count);
      if !copied.next_permutation() {
        break
      }
    }
    if !sorted.next_permutation() {
      break
    }
  }
  println!("{}", max);
}