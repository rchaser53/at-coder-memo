/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    vals:[usize;4]
  }

  let total = vals.iter().sum::<usize>();
  let limit = 1 << 4;
  for i in 1..limit-1 {
    let mut temp = 0;
    for j in 0..limit {
      if i >> j & 1 == 1 {
        temp += vals[j];
      }
    }
    if total == 2 * temp {
      println!("Yes");
      return
    }
  }
  println!("No");
}