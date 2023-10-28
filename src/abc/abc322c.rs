/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;m]
  }

  let mut a = a.into_iter().collect::<VecDeque<usize>>();
  for i in 1..=n {
    while a[0] < i {
      a.pop_front();
    }
    println!("{}", a[0] - i);
  }
}