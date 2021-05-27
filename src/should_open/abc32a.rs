#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

// 最大公約数
fn gcv(a: usize, b: usize) -> usize { 
  if b == 0 {
    a
  } else {
    gcv(b, a % b)
  }
}

// 最小公倍数
fn lcm(a:usize, b:usize) -> usize {
  a * b / gcv(a, b)
}

fn main() {
  input!{
    a: usize,
    b: usize,
    n: usize
  }
  let mut v = lcm(a, b);
  let mut i = 1;
  while v * i <  n {
    i += 1;
  }
  println!("{}", v * i);
}