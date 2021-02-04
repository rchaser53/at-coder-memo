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
    base:[usize;10],
    n: usize,
    vals: [Chars;n]
  }
  
  let mut defs = vec![0;10];
  for i in 0..10 {
    defs[base[i]] = i;
  }
  
  let mut nums = vec![(0,0);n];
  for i in 0..n {
    let s = &vals[i];
    let mut d = 0;
    for ii in (0..s.len()).rev() {
      let iii = s[ii].to_string().parse::<usize>().unwrap();
      nums[i].0 += defs[iii] * 10usize.pow(d);
      d += 1;
    }
    nums[i].1 = i;
  }
  
  nums.sort_by(|a, b| a.0.cmp(&b.0));
  for v in nums {
    let i = v.1;
    println!("{}", vals[i]
      .iter()
      .map(|v| v.to_string())
      .collect::<String>()
    );
  }
}