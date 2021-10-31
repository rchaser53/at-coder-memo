/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    mut red:usize,
    mut green:usize,
    mut blue:usize,
    k:usize
  }

  let mut count = 0;
  while green <= red {
    green *= 2;
    count += 1;
  }

  while blue <= green {
    blue *= 2;
    count += 1;
  }

  if count <= k {
    println!("Yes");
  } else {
    println!("No");
  }
}