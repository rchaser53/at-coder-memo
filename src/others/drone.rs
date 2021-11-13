#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    s: Chars,
    t: usize,
  }
  
  let mut x = 0isize;
  let mut y = 0isize;
  let mut count = 0isize;
  for c in s {
    match c {
      'L' => x -= 1,
      'R' => x += 1,
      'U' => y += 1,
      'D' => y -= 1,
      _ => count += 1
    }
  }
  
  let v = x.abs() + y.abs();
  if t == 1 {
    println!("{}", v + count);
  } else {
    if v <= count {
      if (count - v) % 2 == 1 {
        println!("1");
      } else {
        println!("0");
      }
    } else {
      println!("{}", v - count);
    }
  }
}
