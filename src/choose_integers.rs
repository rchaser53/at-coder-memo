#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashSet, HashMap, VecDeque};
 
fn main() {
  input!{
    a: usize,
    b: usize,
    c: usize
  }
  let mut set: HashSet<usize> = HashSet::new();
  let mut v = a;
  let mut d = 0;
  while !set.contains(&d) {
    let d = v % b;
    if d == c {
      println!("YES");
      return
    }
    set.insert(d);
    v += a;
  }
  println!("NO");
}