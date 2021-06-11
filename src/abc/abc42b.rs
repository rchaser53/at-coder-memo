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
    n:usize,
    l:usize,
    mut vals:[String;n]
  }

  vals.sort();
  let mut result = String::from("");
  for v in vals {
    result = format!("{}{}", result, v);
  }
  
  println!("{}", result);
}