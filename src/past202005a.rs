#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_000;

fn main() {
  input!{
    s: Chars,
    t: Chars,
  }
  
  let mut flag = false;
  for i in 0..3 {
    if s[i] != t[i] {
      if s[i].to_string().to_lowercase()
        == t[i].to_string().to_lowercase() {
        flag = true; 
      } else {
        println!("different");
        return
      }
    }
  }
  
  if flag {
    println!("case-insensitive");
  } else {
    println!("same");
  }
}
