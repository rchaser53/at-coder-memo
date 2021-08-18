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
    n:u32,
  }

  let a = n.to_string();
  let mut temp = 0;
  for c in a.chars() {
    temp += c.to_digit(10).unwrap();
  }

  if n % temp == 0 {
    println!("Yes");
  } else {
    println!("No");
  }
}
