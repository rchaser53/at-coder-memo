/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn solve(s: &[char], t: &[char]) -> usize {
  let mut dp = vec![vec![0;t.len()+1];s.len()+1];
  for i in 0..s.len() {
    for j in 0..t.len() {
      if s[i] == t[j] {
        dp[i+1][j+1] = dp[i][j] + 1;
      } else {
        dp[i+1][j+1] = std::cmp::max(dp[i][j+1], dp[i+1][j]);
      }
    }
  }
  dp[s.len()][t.len()]
}
fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut result = 0;
  for i in 1..n {
    result = std::cmp::max(result, solve(&s[0..i], &s[i..n]));
  }
  result = n - result * 2;
  println!("{}", result);
}