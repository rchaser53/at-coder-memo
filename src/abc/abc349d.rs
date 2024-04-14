/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    mut l:usize,
    r:usize
  }

  let dict = (0..=60).into_iter().map(|v| 2usize.pow(v)).collect::<Vec<usize>>();
  let mut result = vec![];
  while l < r {
    for &v in dict.iter().rev() {
      if l % v == 0 && l + v <= r {
        result.push((l, l+v));
        l += v;
        break
      }
    }
  }

  println!("{}", result.len());
  for (v1, v2) in result {
    println!("{} {}" ,v1, v2);
  }
}