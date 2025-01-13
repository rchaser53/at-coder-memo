/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    mut n:usize,
    mut a:[usize;n]
  }
  a.sort();
  let mut left = 0;
  let mut right = n/2 + 1;
  while left + 1 < right {
    let mid = (left+right)/2;
    let mut success = true;

    let mut tn = n-1;
    for i in (0..mid).rev() {
      let lv = a[i];
      let rv = a[tn];
      tn -= 1;
      if rv < lv * 2 {
        success = false;
        break
      }
    }
    if success {
      left = mid;
    } else {
      right = mid;
    }
  }
  println!("{}", left);
}