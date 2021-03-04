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

fn main() {
  input!{
    mut s: Chars,
    vals: [usize;4]
  }
  
  for i in (0..4).rev() {
    let ti = vals[i];
    s.insert(ti, '"');
  }
  println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
}