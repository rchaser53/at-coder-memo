/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    h:usize,
    w:usize,
    reds:[[usize;w];h],
    blues:[[usize;w];h]
  }

  let limit = 20000;
  let hlimit = limit / 2;
  let mut dp = vec![vec![vec![false;limit];w];h];

  dp[0][0][hlimit+reds[0][0]-blues[0][0]] = true;
  dp[0][0][hlimit-reds[0][0]+blues[0][0]] = true;
  for i in 0..h {
    for j in 0..w {
      let rv = reds[i][j];
      let bv = blues[i][j];

      for k in 0..limit {
        let ai = std::cmp::min(limit-1, (k+rv).saturating_sub(bv));
        let bi = std::cmp::min(limit-1, (k+bv).saturating_sub(rv));
        if 0 < i && dp[i-1][j][k] {
          dp[i][j][ai] = true;
          dp[i][j][bi] = true;
        }
        if 0 < j && dp[i][j-1][k] {
          dp[i][j][ai] = true;
          dp[i][j][bi] = true;
        }
      }
    }
  }

  let mut min = 1_000_000_000;
  for i in 0..limit {
    if dp[h-1][w-1][i] {
      let ii = i as isize;
      min = std::cmp::min(min, (ii - hlimit as isize).abs());
    }
  }
  
  println!("{}", min);
}