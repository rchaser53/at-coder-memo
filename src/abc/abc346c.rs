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
    k:usize,
    a:[usize;n]
  }

  let mut result = (1+k) * k / 2;
  let mut set = HashSet::new();
  for v in a {
    if v <= k {
      set.insert(v);
    }
  }
  for v in set {
    result -= v;
  }
  println!("{}", result);
}