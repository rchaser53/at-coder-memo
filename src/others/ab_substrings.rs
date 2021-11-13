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
    vals: [Chars;n]
  }
  let mut x_a = vec![];
  let mut b_a = vec![];
  let mut b_x = vec![];
  let mut x_x = vec![];
  
  for v in vals {
    if v[0] == 'B' && v[v.len()-1] == 'A' {
      b_a.push(v);
    } else if v[0] == 'B' {
      b_x.push(v);
    } else if v[v.len()-1] == 'A' {
      x_a.push(v);
    } else {
      x_x.push(v);
    }
  }
  
  let mut que = VecDeque::new();
  
  while let Some(v) = b_a.pop() {
    que.push_back(v);
  }
  if let Some(v) = x_a.pop() {
    que.push_front(v);
  }
  if let Some(v) = b_x.pop() {
    que.push_back(v);
  }
  
  while !x_a.is_empty() || !b_x.is_empty() {
    if let Some(v) = x_a.pop() {
      que.push_back(v);
    }
    if let Some(v) = b_x.pop() {
      que.push_back(v);
    }
  }
  while let Some(v) = x_x.pop() {
    que.push_back(v);
  }
  
  let mut last = 'a';
  let mut result = 0;
  while let Some(arr) = que.pop_front() {
    for c in arr {
      if last == 'A' && c == 'B' {
        result += 1;
      }
      last = c;
    }
  }
  
  println!("{}", result);
}