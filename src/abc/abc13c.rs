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
    h:usize,
    a:usize,
    mut b:usize,
    c:usize,
    mut d:usize,
    e:usize
  }
  
  b += e;
  d += e;
  let mut e = e * n;
  if e <= h {
    println!("0");
    return
  }
  e -= h;

  let day = e / d + 1;
  let mut min = day * c;
  for i in 0..day {
    let left = e - i * d;
    let ad = left / b + 1;
    min = std::cmp::min(min, ad * a + i * c);
  }
  println!("{}", min);
}