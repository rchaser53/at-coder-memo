#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }
  vals.sort();
  let mut seen = HashSet::new();
  let mut backs = HashSet::new();
  for i in 0..n {
    backs.insert(vals[i]);
  }
  
  let max = *vals.last().unwrap();
  let mut count = 0;
  
  for i in 0..n {
    let mut v = vals[i];
    if seen.contains(&v) { continue }

    while v <= max {
      if backs.contains(&v) {
        seen.insert(v);
      }
      v *= 2;
    }
    count += 1;
  }
  println!("{}", count);
}
