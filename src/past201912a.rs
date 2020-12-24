#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
 
const MOD:usize = 1_000_000_007;
 
fn main() {
  input!{
    s: String,
  }
  
  if let Ok(v) = s.parse::<usize>() {
    println!("{}", v * 2);  
  } else {
    println!("error");
  }
}