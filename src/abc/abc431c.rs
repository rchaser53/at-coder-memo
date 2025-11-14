/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    mut k:isize,
    mut h:[usize;n],
    mut b:[usize;m]
  }

  h.sort();
  b.sort();
  b.reverse();
  for hv in h {
    while let Some(bv) = b.pop() {
      if hv <= bv {
        k -= 1;
        break
      }
    }
  }
  
  if k <= 0 {
    println!("Yes");
  } else {
    println!("No");
  }
}