#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    vals: [isize;n]
  }
  
  let total = vals.iter().sum::<isize>();
  let mut min = total;
  let mut l = 0;
  let mut r = 0;
  let mut temp = vals[0];
  while r < 2 * n - 1 {
    min = std::cmp::min(min, (2*temp - total).abs());
    if temp <= total - temp && r - l < n {
      r += 1;
      temp += vals[r % n];
    } else {
      if l == r {
        l = r+1;
        r = l;
        temp = vals[r % n];
      } else {
        temp -= vals[l % n];
        l += 1;
      }
    }
  }
  min = std::cmp::min(min, (2*temp - total).abs());
  
  println!("{}", min);
}