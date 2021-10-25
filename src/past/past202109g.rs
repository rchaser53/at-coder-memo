/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[(usize,usize,usize);n]
  }

  let mut left = 0;
  let mut right = usize::max_value();
  while left + 1 < right {
    let mid = (left + right) / 2;
    let mut count = 0;
    for &(a, b, c) in &vals {
      if mid < b { continue }
      let num = (mid - b) / c + 1;
      count += std::cmp::min(a, num);
    }

    if k <= count {
      right = mid;
    } else {
      left = mid;
    }
  }
  println!("{}", right);
}