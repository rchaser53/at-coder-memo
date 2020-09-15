use proconio::input;
use std::cmp::min;
 
fn main() {
  input! {
    n: usize,
    w: usize,
    vals: [(usize, usize);n]
  }
 
  let mw = 100_001;
  let mut dp: Vec<Vec<usize>> = vec![vec![1<<60;mw];n+1];
  dp[0][0] = 0;
  
  for i in 0..n {
    for ii in 0..mw {
      let (weight, value) = vals[i];
      if value <= ii {
        dp[i+1][ii] = min(dp[i][ii-value] + weight, dp[i+1][ii]);
      }
      dp[i+1][ii] = min(dp[i][ii], dp[i+1][ii]);
    }
  }
  let mut max = 0;
  for i in 0..mw {
    if dp[n][i] <= w {
      max = i;
    }
  }
  println!("{}", max);
}