/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
  }

  let mut result = 0;
  let mut a = 1;
  while a*a*a <= n {
    for b in a..=n {
      if b <= n/a/b {
        result += n/a/b - b + 1;
      } else {
        break
      }
    }
    a += 1;
  }
  println!("{}", result);
}