#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    fss: [[usize;10];n],
    pss: [[isize;11];n]
  }
  
  let all = 2usize.pow(10);
  let mut max = -1_000_000_007;
  for i in 1..all {
    let mut count = 0;
    let mut memo: Vec<usize> = vec![0;n];
    for ii in 0..10 {
      if i >> ii & 1 == 1 {
        for fsi in 0..n {
          if fss[fsi][ii] == 1 {
            memo[fsi] += 1;
          }
        }
      }
    }
    let mut temp = 0;
    for i in 0..n {
      let vi = memo[i];
      temp += pss[i][vi];
    }
    max = std::cmp::max(max, temp);
  }
  println!("{}", max);
} 
