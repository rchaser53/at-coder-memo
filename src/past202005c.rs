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
    mut a: u128,
    r: u128,
    n: u32
  }
  
  let limit = 10u128.pow(9);
  if r == 1u128 {
    if a <= limit {
      println!("{}", a);
    } else {
      println!("large");
    }
  } else {
    for i in 1..n {
      a *= r;
      if limit < a {
        println!("large");
        return
      }
    }
    println!("{}", a);
  }
}