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
    n:usize,
    k:usize,
    vals:[(char, Usize1);k]
  }

  let mut counts = vec![k;n];
  let mut memo = vec![false;n];
  for (t, ti) in vals {
    if t == 'L' {
      memo[ti] = true;
      counts[ti] = 1;
      for i in 0..ti {
        if !memo[i] {
          counts[i] -= 1;
        }
      }
    } else {
      memo[ti] = true;
      counts[ti] = 1;
      for i in ti+1..n {
        if !memo[i] {
          counts[i] -= 1;
        }
      }
    }
  }

  let mut result = 1usize;
  for v in counts {
    result *= v;
    result %= MOD;
  }
  println!("{}", result);
}
