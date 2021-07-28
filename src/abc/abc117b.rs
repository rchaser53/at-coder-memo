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
    n:usize,
    mut vals: [usize;n]
  }
  vals.sort();
  let mut count = 0;
  for i in 0..n-1 {
    count += vals[i];
  }

  if vals[n-1] < count {
    println!("Yes");
  } else {
    println!("No");
  }
}
