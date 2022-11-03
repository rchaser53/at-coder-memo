/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
  use proconio::input;
  use proconio::marker::*;
  use std::cmp::Ordering;
  use std::collections::*;
  
  const MOD:usize = 998244353;
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
  
  // 確率DP 確率 MOD
  fn main() {
    input! {
      n:usize,
      m:usize,
      k:usize
    }
  
    let mut result = 0;
    let mut memo = vec![0;n+1];
    memo[0] = 1;
    let mv = modinv(m);
    for _ in 0..k {
      let mut new_memo = vec![0;n+1];
      for i in 0..n {
        for j in 1..=m {
          let ni = if n < i+j {
            n - (i+j-n)
          } else {
            i+j
          };
  
          new_memo[ni] += memo[i] * mv;
          new_memo[ni] %= MOD;
        }
      }

      result += new_memo[n];
      result %= MOD;
      
      memo = new_memo;
    }
  
    println!("{}", result);
  }