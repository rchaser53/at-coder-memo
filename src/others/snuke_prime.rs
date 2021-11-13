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
    c: isize,
    vals:[(isize, isize, isize);n]
  }
  let mut memo = vec![];
  for i in 0..n {
    let (from, to, val) = vals[i];
    memo.push((from-1, val));
    memo.push((to, -val));
  }
  memo.sort();
  let mut cur = 0;
  let mut result = 0;
  let mut now = 0;
  for (day, val) in memo {
    if day != now {
      result += std::cmp::min(cur, c) * (day - now);
      now = day;
    }
    cur += val;
  }
  println!("{}", result);
}