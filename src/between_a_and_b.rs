#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    a: i128,
    b: i128,
    x: i128
  }
  
  let a = if a == 0 {
    println!("{}",
      b / x - a / x + 1
    );
  } else {
    println!("{}",
      b / x - (a-1) / x
    );
  };
}