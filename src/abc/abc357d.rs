/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn repeat_square(n:usize, p:usize, m:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2, m).pow(2) % m 
  } else {
    n * repeat_square(n, p-1, m) % m
  }
}
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
  }

  if n == 1 {
    println!("1");
    return
  }

  let num = n.to_string().len();
  let r = 10usize.pow(num as u32) % MOD;
  let pv = mod_inv((r-1) % MOD);
  let cv = (repeat_square(r, n, MOD) + MOD - 1) % MOD;
  println!("{}", n % MOD * cv % MOD * pv % MOD);
}