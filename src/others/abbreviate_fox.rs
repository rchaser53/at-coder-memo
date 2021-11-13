#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    _n: usize,
    mut s: Chars
  }
  
  let mut i = 0;
  while i + 2 < s.len() {
    if s[i] == 'f'
      && s[i+1] == 'o'
      && s[i+2] == 'x' {
      s.drain(i..=i+2);
      if 2 < i {
        i -= 2;
      } else {
        i = 0;
      }
    } else {
      i += 1;
    }
  }
  println!("{}", s.len());
}
