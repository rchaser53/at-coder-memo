/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

fn culc(s: &Vec<char>) -> bool {
  let n = s.len();
  for i in 0..n {
    if s[i] != s[n-1-i] {
      return false
    }
  }
  true
}

pub fn main(
) {
  input! {
    s:Chars
  }

  let n = s.len();
  let hn = n/2;
  
  let mut a = vec![];
  let mut b = vec![];

  for i in 0..hn {
    a.push(s[i]);
  }
  for i in hn+1..n {
    b.push(s[i]);
  }

  let an = a.len();
  for i in 0..an {
    if a[i] != b[an-i-1] {
      println!("No");
      return
    }
  }

  if culc(&s) && culc(&a) && culc(&b) {
    println!("Yes");
  } else {
    println!("No");
  }
}