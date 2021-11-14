/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n]
  }

  let mut left = 0;
  let mut right = (1 << 60) / k;
  while left + 1 < right {
    let mid = (left + right) / 2;
    let mut tot = 0usize;
    for &v in &vals {
      tot += std::cmp::min(v, mid);
    }

    if mid * k <= tot {
      left = mid;
    } else {
      right = mid;
    }
  }
  println!("{}", left);
}