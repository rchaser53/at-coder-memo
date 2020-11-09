#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(isize, isize);n]
  }
  
  let mut stack = vec![(0,0);n];
  for i in 0..n {
    let (l, r) = vals[i];
    stack[i] = (i, l+r);
  }
  
  let mut a = 0;
  let mut b = 0;
  stack.sort_by(|a,b| a.1.cmp(&b.1));
  stack.reverse();
  for i in 0..n {
    if i % 2 == 0 {
      a += vals[stack[i].0].0;
    } else {
      b += vals[stack[i].0].1;
    }
  }
  
  println!("{}", a - b);
}