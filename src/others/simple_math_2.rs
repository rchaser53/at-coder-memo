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

// m進数を求めるのに使える
fn pow(
  mut x: usize,
  mut n: usize,
  m: usize
) -> usize {
  let mut r = 1;
  while n != 0 {
    if n & 1 == 1 {
      r = r * x % m;
    }
    x = x * x % m;
    n >>= 1;
  }
  r
}

fn main() {
  input!{
    n: usize,
    m: usize,
  }
  
  // 何桁目まで求めたいか
  // この問題の場合、mで割った後にmの余りを求めれば良いからm^2
  let digit = 2u32;
  println!("{}", pow(10, n, m.pow(digit) / m % m);
}