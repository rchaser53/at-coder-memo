/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

fn culc(a:char, b:char) -> bool {
  if a == b {
    true
  } else if a == 'P' {
    if b == 'S' {
      false
    } else {
      true
    }
  } else if a == 'S' {
    if b == 'R' {
      false
    } else {
      true
    }
  } else {
    if b == 'P' {
      false
    } else {
      true
    }
  }
}

pub fn main(
) {
  input! {
    _n:usize,
    mut k:usize,
    mut s:Chars
  }

  while 1 < k {
    if s.len() % 2 == 1 {
      for i in 0..s.len() {
        s.push(s[i]);
      }
    }
    let mut new_s = vec![];
    for i in 0..s.len() / 2 {
      let a = s[i*2];
      let b = s[i*2+1];
      if culc(a, b) {
        new_s.push(a);
      } else {
        new_s.push(b);
      }
    }
    s = new_s;
    k -= 1;
  }

  if s.len() % 2 == 1 {
    for i in 0..s.len() {
      s.push(s[i]);
    }
  }
  
  if culc(s[0], s[1]) {
    println!("{}", s[0]);
  } else {
    println!("{}", s[1]);
  }
}