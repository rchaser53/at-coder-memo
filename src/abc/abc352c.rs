/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    ab: [(usize,usize);n]
  }

  let mut a_tot = 0;
  for i in 0..n {
    a_tot += ab[i].0;
  }

  let mut result = 0;
  for i in 0..n {
    let (a,b) = ab[i];
    result = result.max(a_tot - a + b);
  }

  println!("{}", result);
}