#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

const MOD:usize = 1_000_000_007;
 
#[fastout]
fn main() {
  input! {
    h: usize,
    w: usize,
    kk: usize
  }
  
  let mut dp: Vec<Vec<usize>> = vec![vec![0;w];h+1];
  dp[0][0] = 1;
  
  for r in 0..h {
    for c in 0..w {
      for k in 0..(1 << (w-1)) {
        let mut ok = true;
        
        let w = if w < 2 {
          2
        } else {
          w
        };
        
        for l in 0..(w-2) {
          if (k >> l) & 1 == 1 && k >> l+1 & 1 == 1 {
            ok = false;
          }
        }
        
        if !ok { continue }
        if c >= 1 && k >> (c - 1) & 1 == 1 {
          dp[r+1][c-1] += dp[r][c];
          dp[r+1][c-1] %= MOD;
        }
        else if c <= w-2 && k >> c & 1 == 1 {
          dp[r+1][c+1] += dp[r][c];
          dp[r+1][c+1] %= MOD;
        }
        else {
          dp[r+1][c] += dp[r][c];
          dp[r+1][c] %= MOD;
        } 
      }
    }
  }
  
  println!("{}", dp[h][kk-1]);
}