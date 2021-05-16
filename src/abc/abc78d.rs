#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n:usize,
    z:isize,
    w:isize,
    vals:[isize;n]
  }
  
  let pa = (w-vals[n-1]).abs();
  if n == 1 {
    println!("{}", pa);
    return
  }
  
  let pb = (vals[n-1]-vals[n-2]).abs();
  println!("{}", std::cmp::max(pa, pb));
}