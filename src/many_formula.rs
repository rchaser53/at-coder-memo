#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    mut s: Chars,
  }
  
  let len = s.len();
  let vals = s.into_iter()
           .map(|v| v.to_string().parse::<usize>().unwrap())
           .collect::<Vec<usize>>();
  
  let limit = 1 << (len - 1);
  let mut sum = 0;
  
  for i in 0..limit {
    let mut temp = 0;
    let mut ss = vals[0];
    for ii in 0..len-1 {
      if i >> ii & 1 == 1 {
        temp += ss;
        ss = vals[ii+1];
      } else {
        ss = ss * 10 + vals[ii+1];
      }
    }
    temp += ss;
    sum += temp;
  }
  
  println!("{}", sum);
}