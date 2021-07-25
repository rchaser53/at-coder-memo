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
    s:Chars
  }

  let n = s.len();
  let mut dp = vec![vec![0;8];n];
  let diff = String::from("chokudai").chars().collect::<Vec<char>>();
  
  if s[n-1] == 'i' {
    dp[n-1][7] = 1;
  }

  for i in (0..s.len()-1).rev() {
    dp[i][7] = dp[i+1][7];
    if s[i] == 'i' {
      dp[i][7] += 1;
      dp[i][7] %= MOD;
    }
    for j in 0..7 {
      dp[i][j] = dp[i+1][j];
      if s[i] == diff[j] {
        dp[i][j] += dp[i+1][j+1];
        dp[i][j] %= MOD;
      }
    }
  }
  println!("{}", dp[0][0]);
}
