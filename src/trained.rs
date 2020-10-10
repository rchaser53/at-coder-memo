#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [Usize1;n]
  }
  
  let mut set = HashSet::new();
  let mut count = 0;
  let mut index = 0;
  while !set.contains(&index) {
    if index == 1 {
      println!("{}", count);
      return
    }
    set.insert(index);
    index = vals[index];
    count += 1;
  }
  println!("-1");
}