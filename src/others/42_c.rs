#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input! {
    n: usize,
    k: usize,
    vals: [char;k]
  }
  
  for i in n..=100000 {
    let v = i.to_string();
    let mut flag = false;
    for ii in 0..k {
      let c = vals[ii];
      if v.find(c).is_some() {
        flag = true;
        break;
      }
    }
    if flag { continue }
    println!("{}", i);
    return
  }
}