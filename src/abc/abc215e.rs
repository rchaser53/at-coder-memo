/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 998244353;
pub fn main(
) {
  input! {
    _n:usize,
    s:Chars
  }

  let s = s.into_iter().map(|c| (c as u8 - 'A' as u8) as usize).collect::<Vec<usize>>();
  let limit = 1 << 10;
  let mut dp = vec![vec![0;limit];10];
  
  for x in s {
    let mut next = dp.clone();
    next[x][1 << x] += 1;
    for j in 0..limit {
      for k in 0..10 {
        if j >> x & 1 == 1 && x != k { continue }
        next[x][j | (1 << x)] += dp[k][j];
        next[x][j | (1 << x)] %= MOD;
      }
    }
    dp = next;
  }
  
  let mut result = 0usize;
  for i in 0..limit {
    for j in 0..10 {
      result += dp[j][i];
      result %= MOD;
    }
  }
  println!("{}", result);
}
