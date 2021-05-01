#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
 
fn main() {
  input!{
    n:usize,
    l:usize,
    t:usize,
    x:usize,
    vals:[(usize,usize);n]
  }
  let mut total = 0;
  let mut i = 0;
  let mut temp = 0;
  while i < n {
    let (v, b) = vals[i];
    if b < l {
      total += v + temp;
      i += 1;
      temp = 0;
      continue
    }
    
    if t < v {
      println!("forever");
      return
    }

    if t < temp + v {
      total += t + x;
      if v == t {
        total += t + x;
        temp = 0;
      } else {
        temp = v;      
      }
    } else if t == temp + v {
      total += t + x;
      temp = 0;
    } else {
      temp += v;
    }
    i += 1;
  }
  println!("{}", total + temp);
}