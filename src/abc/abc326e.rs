/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

const MOD:usize = 998244353;
fn mod_pow(a: usize, n: usize) -> usize { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, ((n - 1) / 2));
    let ans = (a * k) % MOD;
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
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut result = 0;
  let mod_n = mod_inv(n);
  let mut tot = mod_n;
  for v in a {
    result += v * tot % MOD;
    result %= MOD;
    tot += (tot * mod_n) % MOD;
    tot %= MOD;
  }
  
  println!("{}", result);
}