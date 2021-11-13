#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, VecDeque};

fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  }
  let mut free = 0;
  let mut memo = vec![false;8];
  
  for v in vals {
    if v <= 399 {
      memo[0] = true;
    } else if v <= 799 {
      memo[1] = true;
    } else if v <= 1199 {
      memo[2] = true;
    } else if v <= 1599 {
      memo[3] = true;
    } else if v <= 1999 {
      memo[4] = true;
    } else if v <= 2399 {
      memo[5] = true;
    } else if v <= 2799 {
      memo[6] = true;
    } else if v <= 3199 {
      memo[7] = true;
    } else {
      free += 1;
    }
  }
  
  let mut count = 0;
  for i in memo {
    if i { count += 1 }
  }
  
  if free == 0 {
    println!("{} {}", count, count);
  } else {
    if count == 0 {
      println!("1 {}", free);
    } else {
      println!("{} {}", count, count + free);
    }
  }
}