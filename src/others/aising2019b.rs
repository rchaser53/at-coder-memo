/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      a:usize,
      b:usize,
      vals:[usize;n]
    }

    let mut memo = vec![0;3];
    for v in vals {
      if v <= a {
        memo[0] += 1;
      } else if 1 + a <= v && v <= b {
        memo[1] += 1;
      } else {
        memo[2] += 1;
      }
    }

    println!("{}", std::cmp::min(
      memo[0],
      std::cmp::min(memo[1], memo[2])
    ));
}

