/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    mut td:[(usize,usize);n]
  }
  td.sort_by(|a,b|a.1.cmp(&b.1));

  let limit = 1440;
  let mut dp = vec![0;limit+1];

  for (t,d) in td {
    let mut new_dp = dp.clone();
    for i in 0..d {
      let ni = i+t;
      if d < ni { break }
      new_dp[ni] = new_dp[ni].max(dp[i]+1);
    }
    dp = new_dp;
  }

  let mut result = 0;
  for v in dp {
    result = result.max(v);
  }
  println!("{}", result);
}