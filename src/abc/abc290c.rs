/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    k:usize,
    mut a:[isize;n]
  }

  a.sort();

  let mut now = 0;
  let mut j = 0;
  for i in 0..n {
    if now == a[i] {
      now += 1;
      j += 1;
    }

    if j == k {
      break
    }
  }
  println!("{}", j);
  
}