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
    mut s:usize
  }

  let mut set = HashSet::new();
  let mut c = 1;
  set.insert(s);

  loop {
    c += 1;
    s = if s % 2 == 0 {
      s / 2
    } else {
      3 * s + 1
    };
    if set.contains(&s) {
      println!("{}", c);
      return
    }
    set.insert(s);
  }
}