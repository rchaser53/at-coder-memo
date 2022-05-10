/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n]
  }

  let mut result = 0;
  let mut left = 0;
  let mut right = 0;
  let mut now = 0;

  while right < n {
    while right < n && now < k {
      now += vals[right];
      right += 1;
    }

    while left <= right && k <= now {
      result += n - right + 1;
      now -= vals[left];
      left += 1;
    }

    if right < left {
      left = right;
      now = 0;
      continue
    }
  }

  println!("{}", result);
}