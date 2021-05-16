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
use std::f64::consts::PI;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    a:usize,
    b:usize,
    k:usize,
    l:usize
  }
  
  if l * a <= b {
    println!("{}", k * a);
  } else {
    let v = k / l;
    println!(
      "{}", 
      std::cmp::min(b * (k / l) + a * (k - l * v), b * (k / l + 1))
    );
  }
}