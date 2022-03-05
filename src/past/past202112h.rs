/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    s:Chars,
    t:Chars
  }

  let mut dp = vec![vec![0;t.len()+1];s.len()+1];
  for i in 0..s.len() {
    for j in 0..t.len() {
      if s[i] != t[j] {
        dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i][j] + 1);
      } else {
        dp[i+1][j+1] = dp[i+1][j+1]
                        .max(dp[i+1][j])
                        .max(dp[i][j+1]);
      }
    }
  }
  
  println!("{}", dp[s.len()][t.len()]);
}