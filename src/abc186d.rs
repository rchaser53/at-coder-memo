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
    n: usize,
    mut vals: [isize;n]
  }
  
  vals.sort();
  let mut culc = vec![0;n];
  culc[0] = vals[0];
  for i in 1..n {
    culc[i] = culc[i-1] + vals[i];
  }
  
  let inn = n as isize;
  let mut result = culc[n-1] - vals[0] * inn;
  for i in 1..n {
    let sum = culc[n-1] - culc[i-1];
    let v = vals[i] * (inn - i as isize);
    result += sum - v;
  }
  println!("{}", result);
}

fn main() {
  input!{
    n:usize,
    mut vals:[isize;n]
  }
  vals.sort();
  let mut temp = 0;
  let mut result = 0;
  for i in 1..n {
    temp += vals[i-1];
    result += vals[i] * i as isize - temp;
  }
  println!("{}", result);
}