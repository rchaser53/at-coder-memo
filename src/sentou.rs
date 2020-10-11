#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashSet, HashMap, VecDeque};
 
fn main() {
  input!{
    n: usize,
    t: usize,
    vals: [usize;n]
  }
  
  let mut total = t;
  let mut last = *vals.last().unwrap();
  for i in (0..n-1).rev() {
    let v = vals[i];
    if v + t < last {
      total += t;
    } else {
      total += last - v;
    }
    last = v;
  }

  println!("{}", total);
}