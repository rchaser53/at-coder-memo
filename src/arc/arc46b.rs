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
    n: usize,
    a: usize,
    b: usize
  }
  
  if n <= a {
    println!("Takahashi");
    return
  }
  
  if a == b {
    if n % (a+1) == 0 {
      println!("Aoki");
    } else {
      println!("Takahashi");
    }
    return
  }
  
  
  if a < b {
    println!("Aoki");
  } else {
    println!("Takahashi");
  }  
}
