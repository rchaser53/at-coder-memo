/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

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
  modpow(a, MOD-2) % MOD
}

const MOD:usize = 1_000_000_007;
fn main() {
  input! {
    n:usize,
    p:usize,
    mut a:[usize;n]
  }

  for i in 0..n {
    a[i] %= MOD;
  }
 
  a.sort();
  let mut map = HashMap::new();
  if p == 0 {
    let mut count = 0;
    for i in 0..n {
      let v = a[i];
      if a[i] == 0 {
        count += i;
      } else {
        let cv = p * modinv(v) % MOD;
        if let Some(num) = map.get(&cv) {
          count += num;
        }
      }
      *map.entry(v).or_insert(0) += 1;
    }
 
    println!("{}", count);
    return
  }
 
  let mut result = 0;
  for v in a {
    if v != 0 {
      let cv = p * modinv(v) % MOD;
      if let Some(num) = map.get(&cv) {
        result += num;
      }      
    }
    *map.entry(v).or_insert(0) += 1;
  }
  // a % MOD = P / b
  println!("{}", result);
}