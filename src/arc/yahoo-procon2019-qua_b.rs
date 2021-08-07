/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    vals:[(Usize1,Usize1);3]
  }

  let mut memo = vec![0;4];
  for (a,b) in vals {
    memo[a] += 1;
    memo[b] += 1;
  }

  for v in memo {
    if 3 <= v {
      println!("NO");
      return
    }
  }
  println!("YES");
}
