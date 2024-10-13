/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:[usize;n]
  }

  let limit = 1 << n;
  let mut result = 10usize.pow(15);
  for i in 0..limit {
    let mut a = 0;
    let mut b = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        a += k[j];
      } else {
        b += k[j];
      }
    }
    result = result.min(a.max(b));
  }

  println!("{}", result);
}