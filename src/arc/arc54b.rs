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
    p:f64,
  }
  
  let mut max = p;
  let mut min = 0f64;
  for _ in 0..1000 {
    let m1 = min + (max-min) / 3f64;
    let m2 = min + (max-min) * 2f64 / 3f64;
    let y1 = m1 + p * 2f64.powf(-m1/1.5f64);
    let y2 = m2 + p * 2f64.powf(-m2/1.5f64);
    if y2 < y1 {
      min = m1;
    } else {
      max = m2;
    }
  }
  
  println!("{}", min + p * 2f64.powf(-min/1.5f64));
}