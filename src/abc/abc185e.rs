/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    nvs:[usize;n],
    mvs:[usize;m]
  }

  let mut dp:Vec<Vec<usize>> = vec![vec![n+m;m+1];n+1];
  dp[0][0] = 0;
  for i in 0..=n {
    dp[i][0] = i;
  }
  for j in 0..=m {
    dp[0][j] = j;
  }

  for i in 1..=n {
    for j in 1..=m {
      let diff = if nvs[i-1] != mvs[j-1] {
        1
      } else {
        0
      };
      dp[i][j] = (dp[i-1][j-1] + diff)
      .min(dp[i-1][j] + 1)
      .min(dp[i][j-1] + 1);
    }
  }
  println!("{}", dp[n][m]);
}
