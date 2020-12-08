#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut vals: [Usize1;n]
  }
  
  let mut i = 0;
  let mut v = 0;
  let mut stack = vec![];
  for i in 0..n-1 {
    if vals[i+1] == v {
      for ii in (v..i+1).rev() {
        let temp = vals[ii];
        vals[ii] = vals[ii+1];
        vals[ii+1] = temp;
        stack.push(ii);
      }
      v = i+1;
    }
  }
  
  if stack.len()+1 != n {
    println!("-1");
    return
  }
  
  for i in 0..n {
    if vals[i] != i {
      println!("-1");
      return
    }
  }
  
  for v in stack {
    println!("{}", v+1);
  }
}