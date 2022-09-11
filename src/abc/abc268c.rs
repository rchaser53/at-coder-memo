/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() { 
  input! { 
    n:usize,
    p:[usize;n]
  }

  let mut result = 0;
  let mut memo = vec![vec![0;n];3];

  for i in 0..n {
    memo[0][(p[i] + n - i) % n] += 1;
    memo[1][(p[i] + n - i + 1) % n] += 1;
    memo[2][(p[i] + n - i - 1) % n] += 1;
  }

  for i in 0..n {
    result = std::cmp::max(result, memo[0][i]+memo[1][i]+memo[2][i]);
  }
  println!("{}", result);
}