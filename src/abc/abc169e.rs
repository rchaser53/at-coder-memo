/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    vals:[(usize, usize);n]
  }
  let mut lefts = vec![0;n];
  let mut rights = vec![0;n];
  for i in 0..n {
    lefts[i] = vals[i].0;
    rights[i] = vals[i].1;
  }
  lefts.sort();
  rights.sort();

  if n % 2 == 0 {
    let lv = lefts[n/2] + lefts[n/2-1];
    let rv = rights[n/2] + rights[n/2-1];
    println!("{}", rv-lv+1);
  } else {
    let lv = lefts[n/2];
    let rv = rights[n/2];
    println!("{}", rv-lv+1);
  }
}
