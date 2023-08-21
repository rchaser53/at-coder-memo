/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut a:[isize;n],
  }

  a.sort();
  let tot = a.iter().sum::<isize>();
  let average = tot / n as isize;
  let remainder = tot as usize % n;

  let mut result = 0;
  for i in 0..n-remainder {
    result += (average - a[i]).abs();
  }
  for i in n-remainder..n {
    result += (average+1 - a[i]).abs();
  }

  println!("{}", result/2);
}