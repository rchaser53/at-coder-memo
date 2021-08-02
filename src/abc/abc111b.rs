/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
  }

  let a = vec![111,222,333,444,555,666,777,888,999];
  for v in a {
    if n <= v {
      println!("{}", v);
      return
    }
  }
}