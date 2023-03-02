/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn main() {
  input!{
    n:usize,
    ab:[(usize,usize);n]
  }

  let mut dp = vec![(0,0);n];
  dp[0] = (1,1);

  for i in 1..n {
    let (av, bv) = ab[i];
    if av != ab[i-1].0 {
      dp[i].0 += dp[i-1].0;
      dp[i].0 %= MOD;
    }
    if av != ab[i-1].1 {
      dp[i].0 += dp[i-1].1;
      dp[i].0 %= MOD;
    }

    if bv != ab[i-1].0 {
      dp[i].1 += dp[i-1].0;
      dp[i].1 %= MOD;
    }
    if bv != ab[i-1].1 {
      dp[i].1 += dp[i-1].1;
      dp[i].1 %= MOD;
    }
  }

  println!("{}", (dp[n-1].0 + dp[n-1].1) % MOD);
}