/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    mut m:usize
  }
  let mut p = vec![];
  for i in (0..=10).rev() {
    p.push(i);
  }

  let mut arr = vec![];
  while 0 < m {
    for &i in p.iter() {
      let v = 3usize.pow(i);
      if m % v == 0 {
        arr.push(i);
        m -= v;
        break
      }
    }
  }

  println!("{}", arr.len());
  println!("{}", arr.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
}