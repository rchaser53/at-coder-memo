#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn helper(
  vals: &Vec<Vec<usize>>,
  i: usize,
  v: usize,
) -> bool {
  if i == vals.len() {
    return v == 0
  }

  for ii in 0..vals[i].len() {
    let vv = vals[i][ii];
    if helper(vals, i+1, vv ^ v) {
      return true
    }
  }
  false
}

fn main() {
  input!{
    n: usize,
    k: usize,
    vals:[[usize;k];n]
  }
  
  for i in 0..vals[0].len() {
    if helper(&vals, 1, vals[0][i]) {
      println!("Found");
      return
    }
  }
  println!("Nothing");
}
