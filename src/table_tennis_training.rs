#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;

#[fastout]
fn main() {
  input!{
    mut n: i128,
    mut a: i128,
    mut b: i128,
  }
  n -= 1;
  a -= 1;
  b -= 1;
  
  let v  = (a - b).abs();
  if v % 2 == 0 {
    println!("{}", v / 2);
  } else {
    if a < n - b {
      println!("{}", a + (b - a).abs() / 2 + 1);
    } else {
      println!("{}", n - b + ((n - b) - (n - a)).abs() / 2 + 1);
    }
  }
}