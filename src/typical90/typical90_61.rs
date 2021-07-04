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
    vals:[(usize, usize);n]
  }

  let mut deque = VecDeque::new();
  for (t, v) in vals {
    if t == 1 {
      deque.push_back(v);
    } else if t == 2 {
      deque.push_front(v);
    } else {
      // v -= 1;
      let ti = deque.len() - v;
      println!("{}", deque[ti]);
    }
  }
}