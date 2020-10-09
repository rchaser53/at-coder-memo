#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    mut s: Chars
  }

  if s.len() == 2 {
    println!("1");
    return
  }
  
  if s.len() % 2 == 0 {
    s.pop();
  }
  s.pop();

  let half = s.len() / 2;
  let mut memo: Vec<char> = vec![];
  let mut max = 0;
  for i in 0..half {
    memo.push(s[i]);
    let mut flag = true;
    for ii in 0..memo.len() {
      if memo[ii] != s[ii+i+1] {
        flag = false;
        break
      }
    }
    
    if flag {
      max = (i + 1) * 2;
    }
  }
  println!("{}", max);
}