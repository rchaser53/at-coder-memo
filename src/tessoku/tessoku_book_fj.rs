/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize,
    ab:[(Usize1,Usize1);m]
  }
  if k == 1 {
    println!("{}", m);
    return
  }
 
  let mut memo = vec![vec![0;n+1];n+1];
  for i in 0..n {
    for j in 0..n {
      let mut count = 0;
      for &(a,b) in &ab {
        if i <= a && b <= j {
          count += 1;
        }
      }
      memo[i+1][j+1] = count;
    }
  }

  let mut dp = vec![0;n+1];
  for i in 1..=n-(k-1) {
    dp[i] = memo[1][i];
  }

  for i in 2..k {
    let mut new_dp = vec![0;n+1];
    for j in i..=n-(k-i) {
      for l in j..=n-(k-i) {
        new_dp[l] = new_dp[l].max(dp[j-1] + memo[j][l])
      }
    }
    dp = new_dp;
  }

  let mut result = 0;
  for i in k-1..=n-1 {
    result = result.max(dp[i]+memo[i+1][n]);
  }
  println!("{}", result);  
}