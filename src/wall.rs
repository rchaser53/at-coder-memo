#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn helper(
  patterns: &mut Vec<Vec<usize>>,
  base: usize,
  next: usize,
  val: usize
) {
  if patterns[base][1] < val { return }
  patterns[base][1] = std::cmp::min(
    patterns[base][1],
    val + patterns[next][1]
  );
    
  for i in 0..10 {
    if next != i {
      helper(patterns, base, i, val + patterns[next][i]);
    }
  }
}

fn main() {
  input!{
    h: usize,
    w: usize,
    mut patterns: [[usize;10];10],
    vals: [[isize;w];h]
  }
  
  for i in 0..10 {
    if i != 1 {
      helper(&mut patterns, i, i, 0);
    }
  }

  let mut count = 0;
  for i in 0..h {
    for ii in 0..w {
      let v = vals[i][ii];
      if v >= 0 {
        count += patterns[v as usize][1];
      }
    }
  }
  println!("{}", count); 
}