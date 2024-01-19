/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut mf = vec![0;n];
  let mut mr = vec![0;n];

  mf[0] = 1;
  mr[n-1] = 1;

  for i in 1..n {
    let lv = mf[i-1];
    mf[i] = (lv+1).min(a[i]);
  }

  for i in (0..n-1).rev() {
    let lv = mr[i+1];
    mr[i] = (lv+1).min(a[i]);
  }

  let mut result = 1;
  for i in 0..n {
    result = result.max(mf[i].min(mr[i]));
  }
  println!("{}", result); 
}