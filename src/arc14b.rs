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
    n: usize,
    vals: [Chars;n]
  }
  
  let mut set = HashSet::new();
  let mut last = vals[0].clone();
  set.insert(last.clone());
  for i in 1..n {
    let new = vals[i].clone();
    if set.contains(&new) || last[last.len()-1] != new[0] {
      if i % 2 == 0 {
        println!("LOSE");
      } else {
        println!("WIN");
      }
      return
    }    
    set.insert(new.clone());
    last = new;
  }
  println!("DRAW");
}