/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    a:usize,
    b:usize,
    c:usize,
    d:usize,
  }

  if a <= c && c <= b {
    println!("Yes");
  } else if a <= d && d <= b {
    println!("Yes");
  } else if c <= a && a <= d {
    println!("Yes");
  } else if c <= b && b <= d {
    println!("Yes");
  } else {
    println!("No");
  }
}