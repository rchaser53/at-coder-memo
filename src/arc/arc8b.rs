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
    n: usize,
    m: usize,
    name: Chars,
    kit: Chars
  }
  
  let mut name_map = HashMap::new();
  for c in name {
    *name_map.entry(c).or_insert(0) += 1;
  }
  
  let mut kit_map = HashMap::new();
  for c in kit {
    *kit_map.entry(c).or_insert(0) += 1;
  }
  
  let mut count = 0;
  let mut used = false;
  loop {
    count += 1;
    used = false;
    for (c, v) in &kit_map {
      if let Some(val) = name_map.get_mut(c) {
        used = true;
        if *val <= *v {
          name_map.remove(c);
        } else {
          *val -= v;
        }
      }
    }
    
    if !used { break }
    if name_map.keys().len() == 0 { break }
  }
  
  if used {
    println!("{}", count); 
  } else {
    println!("-1");
  }
}