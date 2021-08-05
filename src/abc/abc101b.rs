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
  }
  let sn = n.to_string();
  let mut temp = 0;
  for c in sn.chars() {
    temp += (c as u8 - '0' as u8) as usize;
  }

  if n % temp == 0 {
    println!("Yes");
  } else {
    println!("No");
  }
}
