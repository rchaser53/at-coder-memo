/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    s:Chars
  }

  let mut result = vec![];
  for i in 0..s.len() {
    if i % 2 == 0 {
      result.push(s[i].to_string());
    }
  }
  println!("{}", result.into_iter().collect::<String>());
}
