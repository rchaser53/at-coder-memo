/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;


pub fn main(
) {
  input! {
    n:usize,
    l:usize,
    vals:[usize;n]
  }

  let inf = 1_000_000_000_000usize;
  let mut set = 0;
  let mut min = inf;
  for i in 0..n-1 {
    let v = vals[i] + vals[i+1];
    if l <= v && v < min {
      min = v;
      set = i+1;
    }
  }
  
  if min == inf {
    println!("Impossible");
    return
  }

  let mut stack = vec![];
  for i in 1..set {
    stack.push(i);
  }
  for i in (set+1..n).rev() {
    stack.push(i);
  }
  stack.push(set);

  println!("Possible");
  for v in stack {
    println!("{}", v);
  } 
}