#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    x: usize,
  }
  
  for h in 1..=3500 {
    for n in 1..=3500 {
      let right = x * n * h;
      let left_a = 4 * h * n - x * n - x * h;
      if left_a == 0 { continue }
      let w = right / left_a;
      if right % left_a == 0
        && w <= 3500
        && 0 < w {
        println!("{} {} {}", h, n, w);
        return
      }
    } 
  }
}