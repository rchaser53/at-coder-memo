/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut l = 1usize;
  for i in 1..=10usize.pow(6) {
    let v = i.pow(3);
    let s = v.to_string().chars().collect::<Vec<char>>();
    let len = s.len();
    let mut success = true;
    for i in 0..len {
      if s[i] != s[len-1-i] {
        success = false;
        break
      }
    }

    if !success { continue }
    if n == v {
      println!("{}", n);
      return
    } else if n < v {
      println!("{}", l);
      return
    }
    l = v;
  }
  println!("{}", l);
}