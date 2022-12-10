/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut left = 0;
  let mut right = 10usize.pow(9)+10;
  while left + 1 < right {
    let mid = (left+right)/2;
    if mid.pow(2) >= n {
      right = mid;
    } else {
      left = mid;
    }
  }

  let num = n - left * left;
  let group = right;

  if group < num {
    println!("{}", num - group + 1);
  } else {
    println!("{}", group - num + 1);
  }
}