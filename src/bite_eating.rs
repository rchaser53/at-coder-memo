#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: isize,
    l: isize,
  }
  
  let mut all = 0;
  for i in 1..=n {
    all += l + i - 1;
  }
  
  let mut min = 200_000;
  let mut ti = 0;
  for i in 1..=n {
    let v = (l + i - 1).abs();
    if v < min {
      ti = i;
      min = v;
    }
  }
  
  println!("{}", all - (l + ti - 1));
}