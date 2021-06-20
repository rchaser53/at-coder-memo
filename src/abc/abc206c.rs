/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut map = HashMap::new();
  for &v in &vals { 
    *map.entry(v).or_insert(0) += 1;
  }

  let mut result = 0usize;
  for v in &vals {
    let num = map.get(v).unwrap();
    result += n - num;
  }
  println!("{}", result / 2);
}