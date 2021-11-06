/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    s:Bytes,
    t:Bytes
  }

  let (t0, t1, t2) = (t[0], t[1], t[2]);
  let mut dp = vec![vec![0;n+1];n];
  for i in 0..n {
    if s[i..].starts_with(&t) {
      dp[i][i+3] = 1;
    }
  }

  for k in 4..=n {
    for l in 0..=n-k {
      let r = l + k;
      let mut c = (l+1..r).map(|m| dp[l][m] + dp[m][r]).max().unwrap();
      if k % 3 == 0 && s[l] == t0 && s[r-1] == t2 {
        for m in (l+1..r).step_by(3) {
          let a = dp[l+1][m];
          let b = dp[m+1][r-1];
          if s[m] == t1 && a * 3 == m - l - 1
            && b * 3 == r - m - 2 {
              c = c.max(a+b+1);
            }
        }
      }
      dp[l][r] = c;
    }
  }

  println!("{}", dp[0][n]);
}