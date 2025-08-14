/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut a:[isize;n]
  }

  a.sort();
  let mut result = 0;

  for i in 0..=n {
    let mut count = 0;
    let x = i as isize;
    for j in 0..n {
      if a[j] >= x {
        count += 1;
      }
    }

    if x <= count {
      result = max(result, x);
    }
  }

  println!("{}", result);
}