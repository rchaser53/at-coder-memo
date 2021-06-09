/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    k:usize,
    s:Chars,
  }

  let mut result = vec![];
  for i in 0..std::cmp::min(k, s.len()) {
    result.push(s[i]);
  }
  let result = result.iter().map(|v| v.to_string()).collect::<String>();
  if k < s.len() {
    println!("{}...", result);
  } else {
    println!("{}", result);
  }
}