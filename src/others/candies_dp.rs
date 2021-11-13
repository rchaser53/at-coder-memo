use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let mut dp: Vec<Vec<usize>> = vec![vec![0;k+2];n];
  for i in 0..=vals[0] {
    dp[0][i] = 1;
  }
  
  for i in 1..n {
    let mut cum = vec![0;k+2];
    for j in 0..=k {
      cum[j+1] = (cum[j] + dp[i-1][j]) % MOD;
    }
    for j in 0..=k { 
      dp[i][j] += MOD + cum[j+1] - if j >= vals[i] {
        cum[j - vals[i]]
      } else {
        cum[0]
      };
      dp[i][j] %= MOD;
    }
  }
  
  println!("{}", dp[n-1][k]);
}