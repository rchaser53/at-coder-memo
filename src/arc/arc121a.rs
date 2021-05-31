/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 

pub fn main(
) {
input! {
    n:usize,
    vals:[(isize,isize);n],
  }
    
  let mut memo = vec![(0,0,0);n];
  for i in 0..n {
    memo[i] = (i, vals[i].0, vals[i].1);
  }
  memo.sort_by(|a,b| a.1.cmp(&b.1));

  let mut stack = vec![];
  stack.push(
    (
      (memo[0].1 - memo[n-1].1).abs(), memo[0].0, memo[n-1].0
    )
  );
  stack.push(
    (
      (memo[0].1 - memo[n-2].1).abs(), memo[0].0, memo[n-2].0
    )
  );
  stack.push(
    (
      (memo[1].1 - memo[n-1].1).abs(), memo[1].0, memo[n-1].0
    )
  );


  memo.sort_by(|a,b| a.2.cmp(&b.2));
  stack.push(
    (
      (memo[0].2 - memo[n-1].2).abs(), memo[0].0, memo[n-1].0
    )
  );
  stack.push(
    (
      (memo[0].2 - memo[n-2].2).abs(), memo[0].0, memo[n-2].0
    )
  );
  stack.push(
    (
      (memo[1].2 - memo[n-1].2).abs(), memo[1].0, memo[n-1].0
    )
  );

  stack.sort_by(|a,b| a.0.cmp(&b.0));
  if stack[4].1 == stack[5].1 && stack[4].2 == stack[5].2 {
    println!("{}", stack[3].0);
  } else {
    println!("{}", stack[4].0);
  }
}