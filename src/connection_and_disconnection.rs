#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    mut s: Chars,
    k: usize
  }

  let mut count = 0;
  let li = s.len()-1;
  if s[0] == s[li] {
    let last = s[li];

    let mut before = 0;
    for i in 0..=li {
      if last == s[i] {
        before += 1;
      } else {
        break
      }
    }

    if before == s.len() {
      println!("{}", k * s.len() / 2);
      return
    }
    
    let mut after = 0;
    for i in (0..=li).rev() {
      if last == s[i] {
        after += 1;
      } else {
        break
      }
    }
    
    let mut lc = s[0];
    for i in before..s.len()-after {
      if s[i] == lc {
        count += 1;
        lc = '1';
      } else {
        lc = s[i];
      }
    }
    
    println!("{}",
      count * k
      + (k-1) * ((before + after) / 2)
      + before / 2
      + after / 2
    );
  } else {
    let mut lc = s[0];
    for i in 1..s.len() {
      if s[i] == lc {
        count += 1;
        lc = '1';
      } else {
        lc = s[i];
      }
    }
    println!("{}", count * k);    
  }
}