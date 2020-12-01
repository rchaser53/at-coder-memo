#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn helper(
  vals: &Vec<usize>,
  current: usize,
) -> usize {
  let mut children = vec![];
  for i in 0..vals.len() {
    if current == vals[i] {
      children.push(
        helper(vals, i+2)
      );
    }
  }
  
  children.sort();
  if children.is_empty() {
    1
  } else {
    1 + *children.first().unwrap() + *children.last().unwrap()
  }
}

fn main() {
  input!{
    n: usize,
    vals: [usize;n-1]
  }
  
  println!("{}", helper(&vals, 1));
}
