/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    vals:[Chars;h]
  }

  let mut count = 0;
  for i in 0..h {
    for j in 0..w {
      if vals[i][j] == '.' {
        count += 1;
      }
    }
  }
  let mut prepare = vec![0;count+1];
  let mut v = 1;
  prepare[0] = 1;
  for i in 1..=count {
    v *= 2;
    v %= MOD;
    prepare[i] = v; 
  }
  
  let mut memo = vec![vec![0;w];h];
  for i in 0..h {
    let mut j = 0;

    if w == 1 {
      if vals[i][0] == '.' {
        memo[i][0] += 1;
      }
      continue
    }
    while j < w {
      if vals[i][j] == '#' {
        j += 1;
        continue
      }
      let start = j;
      while j < w && vals[i][j] == '.' {
        j += 1;
      }
      let count = j - start;
      for k in start..j {
        memo[i][k] += count;
      }
    }
  }
  for i in 0..w {
    let mut j = 0;
    if h == 1 {
      continue
    }
    while j < h {
      if vals[j][i] == '#' {
        j += 1;
        continue
      }
      let start = j;
      while j < h && vals[j][i] == '.' {
        j += 1;
      }
      let count = j - start;
      for k in start..j {
        memo[k][i] += count - 1;
      }
    }
  }

  let mut result = 0;
  for i in 0..h {
    for j in 0..w {
      if vals[i][j] == '#' { continue }
      let ti = count - memo[i][j];
      let add = (prepare[count] + MOD - prepare[ti]) % MOD;
      result += add;
      result %= MOD;
    }
  }
  println!("{}", result);
}