#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let mut max = 0;
  let mut stack = VecDeque::new();
  let mut a = 1;
  for v in vals {
    if v == 0 {
      println!("{}", n);
      return
    }

    stack.push_back(v);
    a *= v;
    if a <= k {
      max = std::cmp::max(max, stack.len());
    } else {
      while k < a && !stack.is_empty() {
        let vv = stack.pop_front().unwrap();
        a /= vv;
      }
      if stack.is_empty() {
        a = 1;
      }
    }
  }
  
  println!("{}", max);
}