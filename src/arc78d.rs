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

const MOD:usize = 998244353;
const MAX:usize = 200_000;

#[fastout]
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
  
  let mut memo = vec![(-1,false);n];
  memo[0] = (0,true);
  let mut stack = vec![0];
  while !stack.is_empty() {
    let ci = stack.pop().unwrap();
    let v = memo[ci].0 + 1;
    for i in 0..neighbors[ci].len() {
      let ni = neighbors[ci][i];
      if -1 < memo[ni].0 { continue }
      memo[ni] = (v, true);
      stack.push(ni);
    }
  }
  
  memo[n-1] = (0, false);
  let mut stack = vec![n-1];
  while !stack.is_empty() {
    let ci = stack.pop().unwrap();
    let v = memo[ci].0 + 1;
    for i in 0..neighbors[ci].len() {
      let ni = neighbors[ci][i];
      if !memo[ni].1 || memo[ni].0 <= v { continue }
      memo[ni] = (v, false);
      stack.push(ni);
    }
  }
  
  let mut a = 0;
  let mut b = 0;
  for (v, f) in memo {
    if f {
      a += 1;
    } else {
      b += 1;
    }
  }
  
  if b < a {
    println!("Fennec");
  } else {
    println!("Snuke");
  }
}