#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize
  }
  
  if k % 2 == 1 {
    let v = n / k;
    println!("{}", v * v * v);
  } else {
    let v = (n + (k / 2)) / k;
    let temp = v * v * v;
    let v = n / k;
    println!("{}", temp + v * v * v);
  }
}