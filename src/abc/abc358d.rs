/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    mut a:[usize;n],
    mut b:[usize;m]
  }
  
  let mut result = 0;
  a.sort();
  b.sort();
  let mut a = a.into_iter().collect::<VecDeque<usize>>();
  let mut i = 0;
  while i < m {
    if a.is_empty() {
      println!("-1");
      return
    }

    let bv = b[i];
    let av = a.pop_front().unwrap();

    if bv <= av {
      result += av;
      i += 1;
    }
  }
  
  println!("{}", result);
}