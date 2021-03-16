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
    mut vals: [usize;n]
  }
  
  let mut count = 0;
  let mut succeeded = true;
  while succeeded {
    for i in 0..n {
      if vals[i] % 2 == 0 {
        vals[i] /= 2;
      } else {
        succeeded = false;
        break
      }
    }
    
    if succeeded {
      count += 1;    
    }
  }
  println!("{}", count);
}