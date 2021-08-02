/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    vals:[Usize1;n]
  }

  let mut a = (1..=n).into_iter().map(|v| v*30000).collect::<Vec<_>>();
  let b = (1..=n).rev().into_iter().map(|v| v*30000).collect::<Vec<_>>();
  for i in 1..n {
    let ti = vals[i];
    a[ti] += i;
  }

  println!("{}", a.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
  println!("{}", b.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}