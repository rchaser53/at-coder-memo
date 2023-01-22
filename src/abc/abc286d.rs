/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    x:usize,
    ab:[(usize,usize);n]
  }

  let mut dp = vec![false;x+1];
  dp[0] = true;
  for (a,b) in ab {
    let mut new_dp = dp.clone();
    for i in 0..x {
      for j in 1..=b {
        let ni = i + j*a;
        if x < ni { break }
        if dp[i] {
          new_dp[ni] = true;
        }
      }
    }
    dp = new_dp;
  }

  if dp[x] {
    println!("Yes");
  } else {
    println!("No");
  }
}