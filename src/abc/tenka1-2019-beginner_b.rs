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
    n:usize,
    mut s:Chars,
    k:Usize1
  }

  let tc = s[k];

  for i in 0..n {
    if s[i] != tc {
      s[i] = '*';
    }
  }
  println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
}