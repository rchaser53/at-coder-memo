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
    a: Chars,
    b: Chars
  }

  if a.len() < b.len() {
    println!("LESS");
  } else if b.len() > a.len() {
    println!("GREATER");
  } else {
    for i in 0..a.len() {
      if a[i] < b[i] {
        println!("LESS");
        return
      } else if a[i] > b[i] {
        println!("GREATER");
        return
      }
    }
    println!("EQUAL");
  }
}
