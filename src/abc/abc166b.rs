#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
  }

  let mut set = HashSet::new();
  for _ in 0..k {
    input! {
      d:usize,
      vals:[usize;d]
    }
    for v in vals {
      set.insert(v);
    }
  }
  println!("{}", n - set.len());
}