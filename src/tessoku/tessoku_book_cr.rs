/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! { 
    n:usize,
    mw:usize,
    vs:[(usize,usize);n]
  }
  
  let inf = mw+1;
  let limit = 1000*100+10;
  let mut dp = vec![inf;limit];

  dp[0] = 0;
  for (w, v) in vs {
    let mut new_dp = dp.clone();
    for i in 0..limit {
      let ni = v+i;
      if limit <= ni { break }
      if dp[i] != inf && dp[i]+w <= mw {
        new_dp[ni] = new_dp[ni].min(dp[i]+w);
      }
    }
    dp = new_dp;
  }

  for i in (0..limit).rev() {
    if dp[i] != inf {
      println!("{}", i);
      return
    }
  }
}