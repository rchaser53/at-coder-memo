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
    xa: f64,
    ya: f64,
    xb: f64,
    yb: f64,
    xc: f64,
    yc: f64,
  }
  
  let (xbb, ybb, xcc, ycc) = (
    xb - xa, yb - ya,
    xc - xa, yc - ya
  );
  
  println!("{}", ((xbb * ycc) - (ybb * xcc)).abs() / 2f64);
}