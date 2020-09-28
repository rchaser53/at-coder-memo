#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    vals: [Chars;n]
  }
  let mut memo: Vec<usize> = vec![0;5];
  for v in vals {
    match v[0] {
      'M' => memo[0] += 1,
      'A' => memo[1] += 1,
      'R' => memo[2] += 1,
      'C' => memo[3] += 1,
      'H' => memo[4] += 1,
      _ => {}
    }
  }
  
  let mut total = 0;
  for i in 0..5 {
    if memo[i] == 0 { continue }
    for ii in i+1..5 {
      if memo[ii] == 0 { continue }
      for iii in ii+1..5 {
        if memo[iii] == 0 { continue }
        total += memo[i] * memo[ii] * memo[iii];
      }
    }
  }
  
  println!("{}", total);
}