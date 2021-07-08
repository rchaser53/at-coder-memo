/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn culc(
  mut dp: Vec<Vec<isize>>,
  n:usize,
  x:isize,
  p:isize
) -> usize {
  for i in 0..n {
    for j in 0..n {
      if dp[i][j] == -1 {
        dp[i][j] = x;
      }
    }
  }

  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        dp[j][k] = std::cmp::min(dp[j][k], dp[j][i] + dp[i][k]);
      }
    }
  }

  let mut temp = 0;
  for i in 0..n {
    for j in i+1..n {
      if dp[i][j] <= p {
        temp += 1;
      }
    }
  }

  temp
}

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    p:isize,
    k:usize,
    vals:[[isize;n];n]
  }

  let inf = 1_000_000_000_000isize;
  let inf_val = culc(vals.clone(), n, inf, p);
  if inf_val == k {
    println!("Infinity");
    return
  }
  
  let mut left = 0;
  let mut right = inf;
  while left + 1 < right {
    let mid = (left+right) / 2;
    let temp = culc(vals.clone(), n, mid, p);

    if temp < k {
      right = mid;
    } else {
      left = mid;
    }
  }
  let max = left;

  let mut left = 0;
  let mut right = inf;
  while left + 1 < right {
    let mid = (left+right) / 2;
    let temp = culc(vals.clone(), n, mid, p);

    if k < temp {
      left = mid;
    } else {
      right = mid;
    }
  }
  let min = right;

  println!("{}", max - min + 1);
}
