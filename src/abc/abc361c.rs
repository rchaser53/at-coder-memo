/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    mut a:[usize;n]
  }

  a.sort();
  let m = n - k;
  let mut result = 10usize.pow(10);

  for i in 0.. {
    if n < m + i { break }
    result = result.min(a[m+i-1] - a[i]);
  }
  println!("{}", result);
}