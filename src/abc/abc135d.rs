/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    mut s: Chars
  }
  let n = s.len();
  let inf = 1_000_000_000_000usize;
  let mut dp = vec![vec![inf;13];n+1];
  dp[0][0] = 1;
  let mut base = 1;
  s.reverse();

  let t_mod = 13;
  for i in 0..n {
    let c = s[i];
    base %= t_mod;

    for j in 0..13 {
      if dp[i][j] == inf { continue }
      if c == '?' {
        for mut v in 0..10 {
          v *= base;
          v %= t_mod;
          v += j;
          v %= t_mod;
          if dp[i+1][v] == inf {
            dp[i+1][v] = dp[i][j];
          } else {
            dp[i+1][v] += dp[i][j];
          }
          dp[i+1][v] %= MOD;
        }
      } else {
        let mut v = (c as u8 - '0' as u8) as usize;
        v *= base;
        v %= t_mod;
        v += j;
        v %= t_mod;
        if dp[i+1][v] == inf {
          dp[i+1][v] = dp[i][j];
        } else {
          dp[i+1][v] += dp[i][j];
        }
        dp[i+1][v] %= MOD;
      }
    }
    base *= 10;
  }

  if dp[n][5] == inf {
    println!("0");
  } else {
    println!("{}", dp[n][5]);
  }
}
