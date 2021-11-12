/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:isize,
    m:isize,
    k:isize
  }

  for i in 0..=n {
    let a = i * m;
    let li = n - i;
    for j in 0..=m {
      let b = j * (li - i);
      if a + b == k {
        println!("Yes");
        return
      }
    }
  }
  println!("No");
}