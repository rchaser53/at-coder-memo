/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use permutohedron::heap_recursive;
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 100_000;

fn digit_sum(mut x:usize) -> usize {
  let mut result = 0;
  while 0 < x {
    result += x % 10;
    x /= 10;
  }
  result
}

pub fn main(
) {
    input! {
      n:usize,
      mut k:usize,
    }
    let default_value = 1_000_000_000_000usize;
    let mut nexts = vec![0;MOD];
    for i in 0..MOD {
      nexts[i] = (i + digit_sum(i)) % MOD;
    }
    let mut stamps = vec![default_value;MOD];
    let mut pos = n;
    let mut count = 0;
    while stamps[pos] == default_value {
      stamps[pos] = count;
      pos = nexts[pos];
      count += 1;
    }
    let cycle = count - stamps[pos];
    if stamps[pos] <= k {
      k = (k-stamps[pos]) % cycle + stamps[pos];
    }
    let mut result = default_value;
    for i in 0..MOD {
      if stamps[i] == k {
        result = i;
      }
    }
    println!("{}", result);
}
