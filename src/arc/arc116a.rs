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
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 
const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;
 
fn main() {
  input!{
    n:usize,
    vals:[usize;n]
  }
  
  for v in vals {
    if v % 2 == 1 {
      println!("Odd");    
    } else {
      if (v / 2) % 2 == 0 {
        println!("Even");      
      } else {
        println!("Same");
      }
    }
  } 
}