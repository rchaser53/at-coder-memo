#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    mut vals: [usize;3]
  }
  vals.sort();
  
  let a = vals[2] - vals[1];
  let b = vals[2] - vals[0];
  
  if a % 2 == b % 2 {
    println!("{}", (a + b) / 2);
  } else if a % 2 == 1 {
    let a = (vals[2] + 1 - vals[1]) / 2;
    println!("{}", a + (b / 2) + 1);
  } else {
    let b = (vals[2] + 1 - vals[0]) / 2;
    println!("{}", (a / 2) + b + 1);
  }
}