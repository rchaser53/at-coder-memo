#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    mut h: isize,
    mut vals: [(isize, isize);n]
  }
  
  let mut a_vals = vals.clone();
  a_vals.sort_by(|a, b| a.0.cmp(&b.0));
  let max = a_vals.last().unwrap().0;
  
  let mut throws: Vec<isize> = vals
    .into_iter()
    .filter(|v| max <= v.1)
    .map(|v| v.1)
    .collect();
  
  throws.sort();
  throws.reverse();
  
  let len = throws.len() as isize;
  for (i, v) in throws.into_iter().enumerate() {
    h -= v;
    if h <= 0 {
      println!("{}", i+1);
      return
    }
  }
  
  let mut left = h / max;
  if h % max != 0 {
    left += 1;
  }
  println!("{}", len + left);
}
