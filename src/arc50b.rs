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
    r:i128,
    b:i128,
    x:i128,
    y:i128
  }
  
  let mut min = 0;
  let mut max = 10i128.pow(18);
  while min + 1 < max {
    let mid = (min+max)/2;
    let rv = r-mid;
    let bv = b-mid;
    
    if 0 <= rv && 0 <= bv && mid <= rv/(x-1) + bv/(y-1) {
      min = mid;
    } else {
      max = mid;
    }
  }
  
  println!("{}", (min+max)/2);
}