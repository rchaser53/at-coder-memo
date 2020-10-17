#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    k: usize,
    s: usize,
  }
  let mut count = 0;
  for x in 0..=k {
    for y in 0..=k {
      if s <= x + y + k && x + y <= s {
        count += 1;
      }
    }
  }
  println!("{}", count);
}