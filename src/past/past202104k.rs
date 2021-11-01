/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
    vals:[(isize,isize);n]
  }

  let default_val = -10_000_000_000isize;
  let mut dp = vec![default_val;100];
  dp[0] = 0;

  for (p, u) in vals {
    let mut new_dp = dp.clone();
    for i in 0..100 {
      if dp[i] == default_val { continue }
      let ii = i as isize;
      let nv = dp[i] + u - p;
      let ni = ((p + ii) % 100) as usize;
      let add = 20 * ((p+ii) / 100);
      new_dp[ni] = std::cmp::max(new_dp[ni], nv + add);
    }
    dp = new_dp;
  }

  let mut result = 0;
  for v in dp {
    result = std::cmp::max(result, v);
  }

  println!("{}", result);
}