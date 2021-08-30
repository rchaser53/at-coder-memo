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
    s:Chars
  }
  let mut set = HashSet::new();
  let len = s.len();
  for c in s {
    set.insert(c);
  }

  if set.len() == len {
    println!("yes");
  } else {
    println!("no");
  } 
}
