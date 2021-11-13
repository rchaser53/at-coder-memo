#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn modpow(mut a: usize, mut n: usize) -> usize {
  let mut res = 1;
  while n > 0 {
    if n & 1 == 1 {
      res = res * a % MOD; 
    }
    a = a * a % MOD;
    n >>= 1;
  }
  res
}

// a^{-1} mod を計算する
fn modinv(a: usize) -> usize {
  modpow(a, MOD-2)
}

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n],
  }
  
  let mut ownself = 0;
  for i in 0..n {
    for ii in i+1..n {
      if vals[i] > vals[ii] {
        ownself += 1;
      }
    }
  }

  let count = k * (k-1) % MOD * modinv(2) % MOD;
  let mut result = ownself * k % MOD;  
  for i in 0..n {
    let mut cycle = 0;
    for ii in 0..n {
      if i != ii && vals[i] > vals[ii] {
        cycle += 1;
      }
    }
    result += cycle * count % MOD;
    result %= MOD;
  }

  println!("{}", result);
}