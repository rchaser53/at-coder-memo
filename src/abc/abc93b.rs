/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    a:isize,
    b:isize,
    k:isize
  }

  let mut set = HashSet::new();
  let aa = std::cmp::min(a+k, b+1);
  for i in a..aa {
    set.insert(i);
  }

  let bb = std::cmp::max(a, b-k+1);

  for i in bb..=b {
    set.insert(i);
  }
  let mut result = set.into_iter().collect::<Vec<isize>>();
  result.sort();
  for v in result {
    println!("{}", v);
  }
}
