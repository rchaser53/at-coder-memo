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
    l:f64,
    x:f64,
    y:f64,
    s:f64,
    d:f64,
  }
  
  if s == d {
    println!("0");
    return
  }
  
  let (forward, back) = if s < d {
    (d-s, l-d+s)
  } else {
    (l-s+d, s-d)
  };
  
  let v1 = forward / (x + y);
  
  let back_speed = y - x;
  let back_speed = if back_speed < 0f64 {
    println!("{}", v1);
    return
  } else {
    back_speed
  };
  
  let v2 = back / back_speed;
  if v1 < v2 {
    println!("{}", v1);
  } else {
    println!("{}", v2);
  } 
}