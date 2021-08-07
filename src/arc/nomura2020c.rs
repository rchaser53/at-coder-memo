/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n+1]
  }

  if n == 0 && vals[0] == 1 {
    println!("1");
    return
  }

  if vals[0] != 0 {
    println!("-1");
    return
  }

  let mut memo = vec![0;n+1];
  for i in (0..n).rev() {
    memo[i] = vals[i+1] + memo[i+1];
  }

  let inf = 1_000_000_000_000_000usize;
  let mut result = 1usize;
  let mut last = 1usize;
  for i in 1..n {
    let mut limit = if inf < last {
      inf
    } else {
      last * 2
    };

    limit = std::cmp::min(limit, memo[i]+vals[i]);
    if limit < vals[i] {
      println!("-1");
      return
    }
    result += limit;
    last = limit - vals[i];
  }

  if last * 2 < vals[n] {
    println!("-1");
    return
  }

  result += vals[n];
  println!("{}", result);
}
