/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn main() {
  input! {
    n:usize,
    m:usize,
  }

  let mut result = 0;
  for i in 0..=60 {
    if m >> i & 1 == 1 {
      if n >> i & 1 == 1 {
        let mut temp = 1;
        for j in 0..i {
          if n >> j & 1 == 1 {
            temp += (1 << j) % MOD;
            temp %= MOD;
          }
        }
        result += temp;
        result %= MOD;
      }

      for j in i+1..=60 {
        if n >> j & 1 == 1 {
          let b = j - 1;
          result += 2usize.pow(b as u32) % MOD;
          result %= MOD;
        }
      }
    }
  }

  println!("{}", result);
}