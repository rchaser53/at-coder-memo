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

fn culc(
  monster: &Vec<(f64, f64, bool)>,
  val: f64
) -> bool {
  let mut arr = vec![(0f64,false);monster.len()];
  let mut used = false;
  for i in 0..monster.len() {
    let (x, y, flag) = monster[i];
    arr[i] = (y - x * val, flag);
  }
  arr.sort_by(|a, b| {
    if a.0 < b.0 {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  });
  
  let mut i = 0;
  let mut count = 0;
  let mut sum = 0f64;
  while count < 5 {
    let (v, flag) = arr[i];
    if !used {
      used = flag;
      sum += v;
      count += 1;
    } else if used && !flag {
      sum += v;
      count += 1;
    }
    i += 1;
  }
  
  0f64 <= sum
}

fn main() {
  input!{
    n: usize,
    m: usize,
    larges: [(f64, f64);n],
    smalls: [(f64, f64);m]
  }
  
  let mut monsters = vec![(0f64, 0f64, false);n+m];
  for i in 0..n {
    monsters[i] = (larges[i].0, larges[i].1, false);
  }
  
  for i in 0..m {
    monsters[i+n] = (smalls[i].0, smalls[i].1, true);
  }
  
  let mut min = 0f64;
  let mut max = 1_000_000_000f64;

  for i in 0..100 {
    let mid = (min+max) / 2f64;
    if culc(&monsters, mid) {
      min = mid;
    } else {
      max = mid;
    }
  }
  println!("{}", max); 
}
