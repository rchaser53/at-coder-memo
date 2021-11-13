#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, VecDeque};

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut map = HashMap::new();
  for v in vals {
    let entry = map.entry(v).or_insert(0);
    *entry += 1;
  }
  
  let mut count = 0;
  for (v, num) in map {
    if v <= num {
      count += num - v;
    } else {
      count += num;
    }
  }
  println!("{}", count);
}
