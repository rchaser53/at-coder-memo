/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn culc(
  dp: &mut Vec<Vec<Vec<f64>>>,
  n: f64,
  c1: usize,
  c2: usize,
  c3: usize
) -> f64 {
  let cv = dp[c3][c2][c1];
  if 0f64 < cv {
    cv
  } else if c1 == 0 && c2 == 0 && c3 == 0 {
    0f64
  } else {
    let mut base = n;
    if 0 < c1 {
      base += culc(dp, n, c1-1, c2, c3) * c1 as f64;
    }

    if 0 < c2 {
      base += culc(dp, n, c1+1, c2-1, c3) * c2 as f64;
    }

    if 0 < c3 {
      base += culc(dp, n, c1, c2+1, c3-1) * c3 as f64;
    }
    dp[c3][c2][c1] = base / (c1+c2+c3) as f64;
    dp[c3][c2][c1]
  }
}

pub fn main(
) {
    input! {
      n:usize,
      vals:[usize;n]
    }

    let mut memo = vec![0;4];
    for v in vals {
      memo[v] += 1;
    }
    let mut dp = vec![vec![vec![0f64;n+1];n+1];n+1];
    
    println!("{}", culc(&mut dp, n as f64, memo[1], memo[2], memo[3]));
}
