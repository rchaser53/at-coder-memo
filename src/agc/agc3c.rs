/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut cloned = vals.clone();
  cloned.sort();

  let mut map = HashMap::new();
  for i in 0..n {
    map.insert(cloned[i], i % 2 == 1);
  }

  let mut result = 0usize;
  for i in 0..n {
    let v = vals[i];
    if let Some(&f) = map.get(&v) {
      if f == (i % 2 == 0) {
        result += 1;
      }
    }
  }
  println!("{}", result / 2);
}