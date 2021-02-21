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
use superslice::Ext;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    x: Chars,
    m: usize
  }
  
  let mut num = vec![];
  let mut d = 0;
  for c in x {
    let n = c as usize - 48;
    num.push(n);
    d = std::cmp::max(d, n);
  }
  num.reverse();
  if num.len() == 1 {
    if d <= m {
      println!("1");
    } else {
      println!("0");
    }
    return
  }
  
  let mut left = d;
  let mut right = m+1;
  while 1 < right - left {
    let p = (left+right)/2;
    let mut n = 0;
    for i in 0..num.len() {
      if m/p < n {
        n = m+1;
      } else {
        n = n*p + num[num.len()-i-1];
      }
    }
    if n <= m {
      left = p;
    } else {
      right = p;
    }
  }
  println!("{}", left-d);
}