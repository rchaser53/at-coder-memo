/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::*;
use std::rc::*;
use std::cell::*;

fn gcd(a:isize, b:isize) -> isize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn main() {
  input! {
    l:isize,
    r:isize
  }

  let rl = std::cmp::min(l+1000, r);
  let ll = if l  < r-1000 {
    r-1000
  } else {
    l+1
  };

  let mut result = 1;
  for i in l..rl {
    for j in (ll..=r).rev() {
      if j <= i || gcd(i, j) != 1 { continue }
      result = std::cmp::max(result, j-i);
    }
  }
  println!("{}", result);
}