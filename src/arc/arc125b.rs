/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 998244353;
pub fn main(
) {
  input! {
    n:usize
  }

  let mut p = 1;
  let mut result = 0;

  while p * p <= n {
    let limit = n / p;
    let v = if p % 2 == 0 {
      if limit % 2 == 0 {
        (limit - p) / 2 + 1
      } else {
        (limit - p + 1) / 2
      }
    } else {
      if limit % 2 == 0 {
        (limit - p + 1) / 2
      } else {
        (limit - p) / 2 + 1
      }
    };

    result += v;
    result %= MOD;
    p += 1;
  }
  println!("{}", result);
}
