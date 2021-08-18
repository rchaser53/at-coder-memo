/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn gcd(a:isize,b:isize) -> isize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

pub fn main(
) {
  input! {
    n:usize,
    k:isize,
    mut vals:[isize;n]
  }

  vals.sort();
  if vals[n-1] < k {
    println!("IMPOSSIBLE");
    return
  }

  if n == 1 {
    if vals[0] == k {
      println!("POSSIBLE");
    } else {
      println!("IMPOSSIBLE");
    }
    return
  }
  
  let mut gcdv = gcd(vals[0], vals[1]);
  for v in vals {
    gcdv = gcd(gcdv, v);
  }

  if k % gcdv == 0 {
    println!("POSSIBLE");
  } else {
    println!("IMPOSSIBLE");
  }
}
