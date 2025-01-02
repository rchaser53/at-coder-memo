/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    k:usize,
    s:Chars,
    t:Chars
  }
  let n = s.len();
  let m = t.len();

  if n == m {
    let mut count = 0;
    for i in 0..n {
      if s[i] != t[i] {
        count += 1;
      }
    }
    if count <= 1 {
      println!("Yes");
    } else {
      println!("No");
    }
  } else if n+1 == m {
    let mut j = 0;
    for i in 0..m {
      if j < n && s[j] == t[i] {
        j += 1;
      }
    }
    if j == n {
      println!("Yes");
    } else {
      println!("No");
    }
  } else if n-1 == m {
    let mut j = 0;
    for i in 0..n {
      if j < m && s[i] == t[j] {
        j += 1;
      }
    }
    if j == m {
      println!("Yes");
    } else {
      println!("No");
    }
  } else {
    println!("No")
  }
}