/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn gcd(a:usize, b:usize) -> usize {
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
    m:usize,
    mut vals: [(usize, usize);m]
  }
  vals.sort_by(|a,b| a.1.cmp(&b.1));
  let mut result = 0;
  let mut g = n;
  for (a, c) in vals {
    let nxg = gcd(g, a);
    result += (g - nxg) * c;
    g = nxg;
  }

  if g != 1 {
    println!("-1");
  } else {
    println!("{}", result);
  }
}
