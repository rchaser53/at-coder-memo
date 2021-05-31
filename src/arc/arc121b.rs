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
 

fn culc(
  a: &Vec<isize>,
  b: isize
) -> usize {
  let ti = match a.binary_search(&b) {
    Ok(ti) => ti,
    Err(ti) => ti
  };
  if a.len() == ti {
    a.len() - 1
  } else if ti == 0 {
    0
  } else {
    if (b - a[ti]).abs() <  (b - a[ti-1]).abs() {
      ti
    } else {
      ti - 1
    }
  }
}

pub fn main(
) {
input! {
    n:usize,
    vals:[(isize,char);2*n],
  }
    
  let mut memo = vec![vec![];3];
  for (v, c) in vals {
    if c == 'R' {
      memo[0].push(v);
    } else if c == 'G' {
      memo[1].push(v);
    } else {
      memo[2].push(v);
    }
  }

  let mut evens = vec![];
  let mut odds = vec![];
  for i in 0..3 {
    if memo[i].len() % 2 == 1 {
      odds.push(i);
    } else {
      evens.push(i);
    }
    memo[i].sort();
  }

  if odds.is_empty() {
    println!("0");
    return
  }

  let mut s1 = vec![(0,0);memo[odds[0]].len()];
  let mut s2 = vec![(0,0);memo[odds[1]].len()];
  let mut min = isize::max_value();
  for &v in &memo[odds[0]] {
    let a = &memo[odds[1]];
    let ti = culc(a, v);
    min = std::cmp::min(min, (v-a[ti]).abs());
  }

  let ei = evens[0];
  if memo[ei].len() < 2 {
    println!("{}", min);
    return
  }

  let a = &memo[ei];
  for i in 0..s1.len() {
    let v = memo[odds[0]][i];
    let ti = culc(a, v);
    s1[i] = ((v-a[ti]).abs(), ti);
  }
  for i in 0..s2.len() {
    let v = memo[odds[1]][i];
    let ti = culc(a, v);
    s2[i] = ((v-a[ti]).abs(), ti);
  }
  s1.sort_by(|a,b| a.0.cmp(&b.0));
  s2.sort_by(|a,b| a.0.cmp(&b.0));

  for i in 0..std::cmp::min(10, s1.len()) {
    let (v1, ti1) = s1[i];
    for j in 0..std::cmp::min(10, s2.len()) {
      let (v2, ti2) = s2[j];
      if ti1 == ti2 { continue }
      min = std::cmp::min(min, v1 + v2);
    }
  }

  println!("{}", min);
}