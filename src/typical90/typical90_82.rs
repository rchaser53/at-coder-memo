/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

const MOD:usize = 1_000_000_007;
fn mod_pow(a: usize, n: usize) -> usize { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, (n - 1) / 2);
    let ans = (a * k) % MOD;
    (ans * k) % MOD
  } else {
    let k = mod_pow(a, n / 2);
    (k * k) % MOD
  }
}

// a^{-1} mod を計算する
fn mod_inv(a:usize) -> usize {
  mod_pow(a, MOD-2)
}

fn helper(a:usize) -> usize {
  if a == 0 { return 0 }
  let mut result = 0;
  for i in 1..=19 {
    let limit = 10usize.pow(i);
    let digit_num = i as usize;
    if a < limit {

      let a1 = 10usize.pow(i-1);
      let an = a % MOD;
      let n = (MOD+a - 10usize.pow(i-1) + 1) % MOD;
      let v = (a1+an) % MOD * mod_inv(2) % MOD * n % MOD * digit_num % MOD;
      result += v;
      result %= MOD;
      break
    }

    let a1 = 10usize.pow(i-1) % MOD;
    let an = (10usize.pow(i) - 1) % MOD;
    let n = (MOD+an - a1 + 1) % MOD;
    let v = (a1+an) % MOD * mod_inv(2) % MOD * n % MOD * digit_num % MOD;
    result += v;
    result %= MOD;
  }

  result
}

fn main() {
  input! {
    l:usize,
    r:usize
  }

  let lv = helper(l-1);
  let rv = helper(r);
  println!("{}", (MOD + rv - lv) % MOD);
}