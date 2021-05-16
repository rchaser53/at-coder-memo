#![allow(unused_imports)]
use proconio::marker::*;
use std::collections::*;
use std::mem;

const MOD:usize = 1_000_000_007;
fn main() {
  macro_rules! read(
    ($ty:ty) => ({
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      s.trim().parse::<$ty>().unwrap()
    })
  );
  
  let n = read!(usize);
  
  let mut stack = vec![];
  let mut max = 0;
  for i in 2..=n {
    println!("? 1 {}", i);
    let v = read!(usize);
    max = std::cmp::max(max, v);
    stack.push((i, v));
  }
  stack.sort_by(|a,b| a.1.cmp(&b.1));
  let (base, _) = stack.pop().unwrap();
  for (i, v) in stack {
    println!("? {} {}", base, i);
    let v = read!(usize);
    max = std::cmp::max(max, v);
  }
  println!("! {}", max);
}