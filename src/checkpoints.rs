#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    starts: [(isize, isize);n],
    checks: [(isize, isize);m],
  }
  
  for (x, y) in starts {
    let mut min = isize::max_value();
    let mut index = 0;
    for i in 0..m {
      let (xx, yy) = checks[i];
      let v = (x-xx).abs() + (y-yy).abs();
      if v < min {
        min = v;
        index = i + 1;
      }
    }
    println!("{}", index);
  }
}