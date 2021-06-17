/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
fn repeat_square(n:usize, p:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2).pow(2) % MOD 
  } else {
    n * repeat_square(n, p-1) % MOD
  }
}

pub fn main(
) {
  input! {
    n:usize,
    k:usize
  }
  
  if n <= 3 {
    if k < n {
      println!("0");
    } else {
      let mut result = 1;
      for i in 0..n {
        result *= k-i;
        result %= MOD;
      }
      println!("{}", result);
    }
    return
  }

  if k < 3 {
    println!("0");
    return
  }
  
  let mut result = k;
  result *= k - 1;
  result %= MOD;
  
  let v = repeat_square(k-2, n-2);
  result *= v;
  result %= MOD;

  println!("{}", result);
}