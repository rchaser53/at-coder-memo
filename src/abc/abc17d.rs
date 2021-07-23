/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[usize;n]
  }

  let mut dp = vec![0;n+1];
  let mut seen = vec![false;m+1];
  dp[0] = 1;
  let mut l = 0;
  let mut cum = 1;
  for i in 0..n {
    while seen[vals[i]] {
      seen[vals[l]] = false;
      cum = (MOD + cum - dp[l]) % MOD;
      l += 1;
    }
    dp[i+1] = cum;
    cum += dp[i+1];
    cum %= MOD;
    seen[vals[i]] = true;
  }
  println!("{}", dp[n]);
}
