/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

// nCkを楽に計算するためのアレ
struct Choose {
  n:usize,
  numerators:Vec<usize>,
  denominators:Vec<usize>,
}

impl Choose {
  fn new(n:usize) -> Choose {
    let mut numerators = vec![0;n+1];
    let mut denominators = vec![0;n+1];
    numerators[n] = n;
    denominators[0] = 1; 

    for i in (0..n).rev() {
      numerators[i] = numerators[i+1] * i % MOD;
    }
    numerators[0] = 1;
    for i in 1..=n {
      denominators[i] = denominators[i-1] * i % MOD;
    }

    Choose {
      n, numerators, denominators
    }
  }

  fn culc(&self, k:usize) -> usize {
    let numerator = self.numerators[self.n-k+1];
    let denominator = self.denominators[k];
    let inv_k = Choose::mod_inv(denominator);
    numerator * inv_k % MOD
  }

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
  fn mod_inv(a: usize) -> usize {
    Choose::modpow(a, MOD-2)
  }  
}

fn main() {
  input! {
    n:usize,
  }
  
  let mut tens = 1;
  for _ in 1..=n {
    tens *= 10usize;
    tens %= MOD;
  }

  let mut eights = vec![0;n+1];
  eights[0] = 1usize;
  for i in 1..=n {
    eights[i] = eights[i-1] * 8 % MOD;
  }

  let mut memo = vec![0;n+1];
  memo[0] = 1;
  for i in 0..n {
    memo[i+1] = memo[i] * (n-i) % MOD;
  }

  let choose = Choose::new(n);
  let mut base = 0;
  for i in 1..=n {
    base += choose.culc(i) * eights[n-i] % MOD;
    base %= MOD;
  }
  for i in 1..=n {
    base += choose.culc(i) * eights[n-i] % MOD;
    base %= MOD;
  }
  
  let v = (base + eights[n]) % MOD;
  let result = (tens + MOD - v) % MOD;

  println!("{}", result);
}