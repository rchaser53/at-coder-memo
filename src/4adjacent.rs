#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n],
  }
  
  let mut memo = vec![0;3];
  for v in vals {
    if v % 4 == 0 {
      memo[0] += 1;
    } else if v % 2 == 0 {
      memo[1] += 1;
    } else {
      memo[2] += 1;
    }
  }

  if memo[2] <= memo[0] {
    println!("Yes");
  } else if memo[2] == memo[0] + 1 && memo[1] == 0 {
    println!("Yes");
  } else {
    println!("No");
  }  
}
