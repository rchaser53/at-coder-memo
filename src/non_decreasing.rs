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

fn main() {
  input!{
    n: usize,
    mut vals: [isize;n]
  }
  
  let &max = vals.iter().max().unwrap();
  let &min = vals.iter().min().unwrap();
  let mut result = vec![];
  if !(0 <= min || max <= 0) {
    let x = if max.abs() <= min.abs() {
      vals.iter().position(|a| a == &min).unwrap()
    } else {
      vals.iter().position(|a| a == &max).unwrap()
    };
    for y in 0..n {
      vals[y] += vals[x];
      result.push((x, y));
    }
  }
  
  let &max = vals.iter().max().unwrap();
  if max <= 0 {
    for y in (0..n-1).rev() {
      vals[y] += vals[y+1];
      result.push((y+1, y));
    }
  } else {
    for y in 1..n {
      vals[y] += vals[y-1];
      result.push((y-1, y));
    }
  }

  println!("{}", result.len());
  for (f, t) in result {
    println!("{} {}", f+1, t+1);
  }
}