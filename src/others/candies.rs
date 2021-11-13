#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    vals: [[usize;n];2],
  }
  
  let mut twos = vec![0;n];
  twos[n-1] = vals[1][n-1];
  for i in (0..n-1).rev() {
    twos[i] = twos[i+1] + vals[1][i];
  }
  
  let mut max:usize = vals[0].iter().sum();
  let mut now = 0;
  for i in 0..n {
    now += vals[0][i];
    let v = now + twos[i];
    max = std::cmp::max(max, v);
  }
  
  println!("{}", max);
}
