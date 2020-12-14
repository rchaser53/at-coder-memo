#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    mut vals: [usize;m]
  }
  
  if n == m {
    println!("0");
    return
  }
  
  let mut stack = vec![];
  vals.push(0);
  vals.push(n+1);
  vals.sort();
  for p in vals.windows(2) {
    let v = p[1] - p[0] - 1;
    if v != 0 {
      stack.push(v);
    }
  }
  
  let mut min = 2_000_000_000;
  for i in 0..stack.len() {
    let v = stack[i];
    min = std::cmp::min(min, v);
  }
  
  let mut count = 0;
  for v in stack {
    count += v / min;
    if v % min != 0 {
      count += 1;
    }
  }
  println!("{}", count);
}