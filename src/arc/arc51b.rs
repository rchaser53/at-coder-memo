#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    k:usize
  }
  
  let mut a = 1;
  let mut b = 1;
  for _ in 0..k {
    let temp = b;
    b = a;
    a += temp;
  }
  println!("{} {}", a, b);
}