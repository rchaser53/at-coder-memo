/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    x:usize,
    y:usize,
    mut a:[usize;n],
    mut b:[usize;n]
  }

  a.sort();
  a.reverse();
  b.sort();
  b.reverse();
  let mut cx = 0;
  let mut cy = 0;
  for i in 0..n {
    cx += a[i];
    cy += b[i];

    if cx > x || cy > y {
      println!("{}", i+1);
      return
    }
  }
  println!("{}", n);
}