#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
 
#[fastout]
fn main() {
  input!{
    a: i128,
    b: i128,
    c: i128
  }
  
  let v = c - a - b;
  
  if v <= 0 {
    println!("No");
    return
  }
  
  if 4 * a * b < v * v {
    println!("Yes");
  } else {
    println!("No");
  }
}