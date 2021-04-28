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
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 
const MOD:usize = 998244353;
const MAX:usize = 400010;

fn main() {
  input!{
    n:usize,
    vals:[(Usize1,Usize1);n-1]
  }
  
  let mut neighbors = vec![vec![];n];
  for (from, to) in vals {
    neighbors[from].push(to);
    neighbors[to].push(from);
  }
  
  let mut seen = HashSet::new();
  let mut memo = vec![vec![0]];
  let mut stack = vec![(0,0)];
  while !stack.is_empty() {
    let (ci, level) = stack.pop().unwrap();
    let ti = level+1;
    seen.insert(ci);

    for i in 0..neighbors[ci].len() {
      let ni = neighbors[ci][i];
      if seen.contains(&ni) { continue }
      if memo.len() <= ti {
        memo.push(vec![]);
      }
      memo[ti].push(ni);
      stack.push((ni, ti));
    }
  }
  
  let half = n / 2;
  let mut odd = vec![];
  let mut even = vec![];
  for i in 0..memo.len() {
    if i % 2 == 0 {
      for j in 0..memo[i].len() {
        if even.len() < half {
          even.push((memo[i][j]+1).to_string());
        }
      }
    } else {
      for j in 0..memo[i].len() {
        if odd.len() < half {
          odd.push((memo[i][j]+1).to_string());
        }
      }
    }
  }
  if half <= odd.len() {
    println!("{}", odd.join(" "));
  } else {
    println!("{}", even.join(" "));
  }
  
}