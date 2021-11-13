#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [isize;n],
  }
  
  let mut memo: Vec<isize> = vec![0;n];
  memo[0] = vals[0];
  for i in 1..n {
    memo[i] = vals[i] + memo[i-1];
  }
  
  let mut min = isize::max_value();
  for i in 0..n-1 {
    min = std::cmp::min(min, (memo[n-1] - 2 * memo[i]).abs());
  }
  println!("{}", min);
}