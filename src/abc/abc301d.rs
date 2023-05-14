/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    mut s:Chars,
    n:usize,
  }

  let m = s.len();
  s.reverse();
  let mut result = 0;
  for i in 0..m {
    if s[i] == '1' {
      result += 2usize.pow(i as u32);
    }
  }

  if n < result {
    println!("-1");
    return
  }

  let limit = (m-1).min(60);
  for i in (0..=limit).rev() {
    let v = 2usize.pow(i as u32);
    let nv = result + v;
    if s[i] == '?' && nv <= n {
      result = nv;
    }
  }
  
  println!("{}", result);
}