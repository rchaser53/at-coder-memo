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

#[fastout]
fn main() {
  input!{
    a: usize,
    b: usize,
    c: usize
  }
  
  let a2 = a % 10;
  let b2 = b % 4;
  let c2 = c % 2;
  if a2 == 0 || a2 == 1 || a2 == 5 || a2 == 6 {
    println!("{}", a2);
    return
  }
  
  let list_a = vec![
    vec![0, 0, 0, 0],
    vec![0, 0, 0, 0],
    vec![6, 2, 4, 8],
    vec![1, 3, 9, 7],
    vec![6, 4, 6, 4],
    vec![0, 0, 0, 0],
    vec![0, 0, 0, 0],
    vec![1, 7, 9, 3],
    vec![6, 8, 4, 2],
    vec![1, 9, 1, 9]
  ];
  let list_b = vec![
    [0, 0],
    [1, 1],
    [0, 2],
    [1, 3],
  ];
  if b2 == 2 {
    if c == 1 {
      println!("{}", list_a[a2][2]);
    } else {
      println!("{}", list_a[a2][0]);
    }
    return
  }

  println!("{}", list_a[a2][list_b[b2][c2]]);
}