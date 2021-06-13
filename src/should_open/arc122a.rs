/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:isize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    vals:[isize;n],
  }

  // 0:+ 1:-
  let mut dp = vec![(0,0);n];
  dp[0] = (1, 0);
  for i in 1..n {
    dp[i].0 = (dp[i-1].0 + dp[i-1].1) % MOD;
    dp[i].1 = dp[i-1].0;
  }

  let mut total = 1;
  for i in 1..n {
    total += dp[i].1;
    total %= MOD;
  }
  let mut result = total * vals[0] % MOD;
  for i in 1..n {
    result = (result + (total + MOD - dp[i-1].0 * dp[n-1-i].0 * 2 % MOD) * vals[i]) % MOD;
  }
  println!("{}", result);
}