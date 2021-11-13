#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn is_prime(a: usize) -> bool {
  let mut i = 2;
  while i * i <= a {
    if a % i == 0 {
      return false;
    }
    i += 1;
  }
  return true;
}

fn main() {
  input!{
    q: usize,
    vals: [(usize, usize);q]
  }
  
  let max = 100000;
  let mut temps: Vec<bool> = vec![false;max];
  
  let mut i = 1;
  while i < max {
    temps[i] = is_prime(i);
    i += 2;
  }
  
  let mut dp: Vec<usize> = vec![0;max];
  let mut temp = 1;
  dp[3] = 1;
  let mut i = 5;
  while i < max {
    if temps[i] && temps[(i+1)/2] {
      temp += 1;
    }
    dp[i] = temp;
    i += 2;
  }
  
  for (l, r) in vals {
    if 2 < l {
      println!("{}", dp[r] - dp[l-2]);
    } else {
      println!("{}", dp[r]);
    }
  }
}
