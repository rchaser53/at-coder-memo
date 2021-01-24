#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }

  vals.push(0usize);
  let mut max = 0;
  let mut stack: Vec<(usize, usize)> = vec![];
  for i in 0..=n {
    let mut ni = i;
    while !stack.is_empty() && vals[i] < stack.last().unwrap().1 {
      let (j, h) = stack.pop().unwrap();
      max = std::cmp::max(max, h * (i - j));
      ni = j;
    }
    if stack.is_empty() || stack.last().unwrap().1 < vals[i] {
      stack.push((ni, vals[i]));
    }
  }
 
  println!("{}", max);
}