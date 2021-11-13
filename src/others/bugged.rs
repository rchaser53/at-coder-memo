#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }
  let mut tens = vec![];
  let mut lefts = vec![];
  for v in vals {
    if v % 10 == 0 {
      tens.push(v);
    } else {
      lefts.push(v);
    }
  }
  let mut result = tens.into_iter().sum::<usize>();

  lefts.sort();
  let mut left_tot = lefts.iter().sum::<usize>();
  if !lefts.is_empty() && left_tot % 10 == 0 {
    left_tot -= lefts.first().unwrap();
  }
  result += left_tot;
  if result % 10 == 0 {
    println!("0");
  } else {
    println!("{}", result);
  }
}