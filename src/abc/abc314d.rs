/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut s:Chars,
    q: usize,
    queries:[(usize,usize,char);q]
  }

  let mut li = 0;
  let mut dirty = false;
  for i in 0..q {
    if queries[i].0 != 1 {
      li = i;
      dirty = true;
    }
  }

  for i in 0..li {
    let (t,ti,c) = queries[i];
    if t == 1 {
      s[ti-1] = c;
    }
  }

  if !dirty {
    let (_t,ti,c) = queries[0];
    s[ti-1] = c;
  }

  if dirty {
    if queries[li].0 == 2 {
      for i in 0..n {
        s[i] = s[i].to_ascii_lowercase();
      }
    } else {
      for i in 0..n {
        s[i] = s[i].to_ascii_uppercase();
      }
    }
  }

  for i in li+1..q {
    let (t,ti,c) = queries[i];
    if t == 1 {
      s[ti-1] = c;
    }
  }

  println!("{}", s.iter().map(|v| v.to_string()).collect::<String>());
}