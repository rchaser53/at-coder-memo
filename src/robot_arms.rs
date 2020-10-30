#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [(isize, isize);n],
  }
  
  vals = vals
    .into_iter()
    .map(|v| (v.0 - v.1, v.0 + v.1))
    .collect();
  vals.sort_by(|a, b| {
    let v = a.1.cmp(&b.1);
    if Ordering::Equal == v {
      a.0.cmp(&b.0)
    } else {
      v
    }
  });
  
  let mut count = 0;
  let mut last = vals[0].1;  
  for i in 1..n {
    let (left, right) = vals[i];
    if left < last {
      count += 1;
    } else {
      last = right;
    }
  }
  
  println!("{}", n - count);
}