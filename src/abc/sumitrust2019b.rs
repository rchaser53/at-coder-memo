/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:f64,
  }

  for i in 1..=50000 {
    let v = (i as f64 * 1.08).floor();
    if v == n {
      println!("{}", i);
      return
    }
  }
  println!(":(");
}