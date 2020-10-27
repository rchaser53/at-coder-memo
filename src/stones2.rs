#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    s: Chars
  }
  
  let mut whites = vec![0;n];
  let mut blacks = vec![0;n];
  
  if s[0] == '#' {
    blacks[0] = 1;
  }
  for i in 1..n {
    if s[i] == '#' {
      blacks[i] = blacks[i-1] + 1;
    } else {
      blacks[i] = blacks[i-1];
    }
  }
  
  if s[n-1] == '.' {
    whites[n-1] = 1;
  }
  for i in (0..n-1).rev() {
    if s[i] == '.' {
      whites[i] = whites[i+1] + 1;
    } else {
      whites[i] = whites[i+1];
    }
  }
  
  let mut min = usize::max_value();
  for i in 0..n-1 {
    min = std::cmp::min(whites[i+1] + blacks[i], min);
  }
  min = std::cmp::min(min, whites[0]);
  min = std::cmp::min(min, blacks[n-1]);
    
  println!("{}", min);
}