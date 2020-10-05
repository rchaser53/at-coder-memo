#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut count = 0;
  let mut index = 0;
  while index < n {
    if vals[index] == index+1 {
      count += 1;
      index += 1;
    }
    index += 1;
  }
  println!("{}", count);
}
