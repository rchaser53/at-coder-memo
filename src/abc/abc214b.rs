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
    s:usize,
    t:usize
  }

  let mut set = HashSet::new();
  for a in 0..=s {
    for b in 0..=s {
      for c in 0..=s {
        if a + b + c <= s && a * b * c <= t {
          set.insert((a,b,c));
        }
      }
    }
  }

  println!("{}", set.len());
}
