/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    s:Chars
  }
  
  let n = s.len();
  let mut dp = vec![vec![0usize;4];n+1];
  dp[0][0] = 1;

  for i in 1..=n {
    let c = s[i-1];
    for j in 0..4 {
      if c == '?' {
        dp[i][j] = 3 * dp[i-1][j];
        dp[i][j] %= MOD;
      } else {
        dp[i][j] = dp[i-1][j];
      }
    }

    if c == 'A' {
      dp[i][1] += dp[i][0];
      dp[i][1] %= MOD;
    } else if c == 'B' {
      dp[i][2] += dp[i-1][1];
      dp[i][2] %= MOD;
    } else if c == 'C' {
      dp[i][3] += dp[i-1][2];
      dp[i][3] %= MOD;
    } else {
      dp[i][3] += dp[i-1][2];
      dp[i][2] += dp[i-1][1];
      dp[i][1] += dp[i-1][0];
      dp[i][3] %= MOD;
      dp[i][2] %= MOD;
      dp[i][1] %= MOD;
    }
  }

  println!("{}", dp[n][3]);
}