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
  let mut memo = vec![vec![0;10];limit];
  for v in s {
    let mut next = memo.clone();
    next[1 << v][v] += 1;
    next[1 << v][v] %= MOD;
    for i in 0..10 {
      for j in 0..limit {
        if j >> v & 1 == 1 && v != i { continue }
        next[j | 1 << i][v] += memo[j][i];
        next[j | 1 << i][v] %= MOD;
      }
    }
    memo = next;
  }

  let mut result = 0;
  for i in 0..10 {
    for j in 0..limit {
      result += memo[j][i];
      result %= MOD;
    }
  }
  println!("{}", result);
}
