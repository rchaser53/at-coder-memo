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
    mut vals: [isize;n]
  }
  
  let mut ti = 0;
  let mut max = 0isize;
  for i in 0..n {
    if max.abs() <= vals[i].abs() {
      max = vals[i];
      ti = i;
    }
  }

  let mut result = vec![];
  for i in 0..n {
    vals[i] += vals[ti];
    result.push((ti, i));
  }
  
  if max < 0 {
    for i in (1..n).rev() {
      vals[i-1] += vals[i];
      result.push((i, i-1));
    }
  } else {
    for i in 0..n-1 {
      vals[i+1] += vals[i];
      result.push((i, i+1));
    }
  }
  
  println!("{}", result.len());
  for (f, t) in result {
    println!("{} {}", f+1, t+1);
  }
}