/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    r:usize,
    c:usize,
    k:usize,
    vals:[(usize,usize,usize);k]
  }

  let mut dp = vec![vec![vec![0;4];c+1];r+1];
  let mut rows = vec![vec![0;c+1];r+1];

  for (r, c, v) in vals {
    rows[r][c] = v;
  }

  for y in 1..=r {
    for x in 1..=c {
      let nv = rows[y][x];
      for i in 0..4 {
        dp[y][x][1] = std::cmp::max(dp[y][x][1], dp[y-1][x][i] + nv);
        dp[y][x][0] = std::cmp::max(dp[y][x][0], dp[y-1][x][i]);
      }

      for i in 0..4 {
        dp[y][x][i] = std::cmp::max(dp[y][x-1][i], dp[y][x][i]);
        if 0 < nv && i < 3 {
          dp[y][x][i+1] = std::cmp::max(dp[y][x-1][i]+nv, dp[y][x][i+1]);
        }
      }
    }
  }

  let mut result = 0usize;
  for i in 0..4 {
    result = std::cmp::max(result, dp[r][c][i]);
  }
  println!("{}", result);
}