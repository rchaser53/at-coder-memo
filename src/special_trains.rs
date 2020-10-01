#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    vals: [(usize, usize, usize);n-1]
  }
  
  for i in 0..n-1 {
    let mut last = vals[i].0 + vals[i].1;
    for ii in i+1..n-1 {
      let (time, start, span) = vals[ii];
      if last <= start {
        last = start + time;
      } else {
        let v = last - start;
        let mut vv = (v / span) * span;
        if v % span != 0 {
          vv += span;
        }
        last = vv + start + time;
      }
    }
    println!("{}", last);
  }
  println!("0");
}
