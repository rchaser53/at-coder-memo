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
    mut vals:[(isize, isize);n]
  }

  vals.sort_by(|a,b| a.0.cmp(&b.0));
  let x_mid = vals[n/2].0;
  let mut xv = 0;
  for &v in &vals {
    xv += (v.0 - x_mid).abs();
  }
 
  vals.sort_by(|a,b| a.1.cmp(&b.1));
  let y_mid = vals[n/2].1;
  let mut yv = 0;
  for &v in &vals {
    yv += (v.1 - y_mid).abs();
  }
  
  println!("{}", xv + yv);

}