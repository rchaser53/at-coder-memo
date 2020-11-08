#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
  }
 
  let mut result = 0;
  let mut i = n;
  let mut stack = vec![];
  let mut v = 2;
  while v * v <= i {
    if i % v == 0 {
      stack.push(v);
      i /= v;
    } else {
      v += 1;
    }
  }
  if 1 < i {
    stack.push(i);
  }
  
  let limit = 2usize.pow(stack.len() as u32);
  let mut seen = HashSet::new();
  for i in 0..limit {
    let mut temp = 1;
    for ii in 0..stack.len() {
      if i >> ii & 1 == 1 {
        temp *= stack[ii];
      }
    }
    if !seen.contains(&temp) && 1 < temp {
      let v = temp - 1;
      if n % v == n / v {
        result += v;
      }
      seen.insert(temp);
    }
  }
  
  println!("{}", result);
}