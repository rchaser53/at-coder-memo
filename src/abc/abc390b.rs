/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  for i in 0..n-2 {
    if a[i] * a[i+2] != a[i+1] * a[i+1] {
      println!("No");
      return
    }
  }

  println!("Yes");
}