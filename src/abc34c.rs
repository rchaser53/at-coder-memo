#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
fn mod_pow(a: usize, n: usize) -> usize { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, ((n - 1) / 2));
    let mut ans = (a * k) % MOD;
    (ans * k) % MOD
  } else {
    let k = mod_pow(a, (n / 2));
    (k * k) % MOD
  }
}

// a^{-1} mod を計算する
fn mod_inv(a:usize) -> usize {
  mod_pow(a, MOD-2)
}

fn main() {
  input!{
    w: usize,
    h: usize
  }
  
  let mut child = 1;
  let max = h+w-2;
  for i in 2..=max {
    child *= i;
    child %= MOD;
  }
  let mut parent_a = 1;
  for i in 2..=max-h+1 {
    parent_a *= i;
    parent_a %= MOD;
  }
  let inv_parent_a = mod_inv(parent_a);
  let mut parent_b = 1;
  for i in 2..=h-1 {
    parent_b *= i;
    parent_b %= MOD;
  }
  let inv_parent_b = mod_inv(parent_b);
  
  let mut result = child * inv_parent_a % MOD;
  println!("{}", result * inv_parent_b % MOD);
}
