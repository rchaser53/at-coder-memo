/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    vals:[(usize,usize);n]
  }

  let limit = 5000;
  let mut memo = vec![vec![0;limit+2];limit+2];
  for (x, y) in vals {
    memo[x][y] += 1;
  }

  for i in 1..=limit {
    for j in 1..=limit {
      memo[i][j] += memo[i-1][j] + memo[i][j-1] - memo[i-1][j-1];
    }
  }

  let mut max = 0;
  for li in 1..=limit {
    let ri = std::cmp::min(limit, li+k);
    for lj in 1..=limit {
      let rj = std::cmp::min(limit, lj+k);
      let v = memo[ri][rj] - memo[li-1][rj] - memo[ri][lj-1] + memo[li-1][lj-1];
      max = std::cmp::max(max, v);
    }
  }
  println!("{}", max);
}
