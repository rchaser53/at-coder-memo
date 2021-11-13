#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

const MOD: usize = 1000000007;

#[fastout]
fn main() {
  input!{
    n: usize,
    row1: Chars,
    row2: Chars,
  }
  
  let mut i = 0;
  let mut count = 0;
  let mut vertical = true;
  if row1[0] == row2[0] {
    count = 3;
    i = 1;
    vertical = true;
  } else {
    count = 6;
    i = 2;
    vertical = false;
  }
  
  while i < n {
    if vertical {
      if row1[i] == row2[i] {
        i += 1;
        vertical = true;
        count *= 2;
      } else {
        i += 2;
        vertical = false;
        count *= 2;
      }
    } else {
      if row1[i] == row2[i] {
        i += 1;
        vertical = true;
        count *= 1;
      } else {
        i += 2;
        vertical = false;
        count *= 3;
      }
    }
    count %= MOD;
  }
  println!("{}", count);
}
