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
    x: f64,
    y: f64
  }
  
  if y == 0f64 {
    println!("ERROR");
  } else {
    let v = x / y;
    let vv = if v.fract() == 0f64 {
      format!("00")
    } else {
      let v = v.fract().to_string();
      if v.len() == 3 {
        format!("{}0", &v[2..3])  
      } else {
        format!("{}", &v[2..4])
      }
    };
    println!("{}.{}", v.trunc() as usize, vv);
  }
}