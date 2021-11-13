#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashSet, HashMap, VecDeque};
 
fn main() {
  input!{
    mut s: Bytes,
    K: usize,
  }
  
  let n = s.len();
  let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0;2];4];105];
  dp[0][0][0] = 1;
  for i in 0..n {
    for j in 0..=K {
      for k in 0..2 {
        let nd = s[i] - b'0';
        for d in 0..10 {
          let mut ni = i+1;
          let mut nj = j;
          let mut nk = k;
          if d != 0 { nj += 1; }
          if nj > K { continue; }
          if k == 0 {
            if d > nd { continue }
            if d < nd { nk = 1; }
          }
          dp[ni][nj][nk] += dp[i][j][k];
        }
      }
    }
  }
  println!("{}", dp[n][K][0] + dp[n][K][1]);
}