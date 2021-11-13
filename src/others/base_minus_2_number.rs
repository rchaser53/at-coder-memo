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

#[fastout]
fn main() {
  input!{
    mut n: isize
  }
  
  if n == 0 {
    println!("0");
    return
  }
  
  let mut result = String::from("");
  while 0 != n {
    if n % 2 != 0 {
      n -= 1;
      result = format!("1{}", result);
    } else {
      result = format!("0{}", result);
    }
    n /= -2;
  }
  println!("{}", result);
}