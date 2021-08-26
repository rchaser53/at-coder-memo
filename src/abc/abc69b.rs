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

  let len = s.len() - 2;
  println!("{}{}{}", s[0], len, s[s.len()-1]);
}
