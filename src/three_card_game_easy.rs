#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    mut sa: Chars,
    mut sb: Chars,
    mut sc: Chars
  }
  
  sa.reverse();
  sb.reverse();
  sc.reverse();
  
  let mut last = sa.pop().unwrap();
  loop {
    if last == 'a' {
      if let Some(v) = sa.pop() {
        last = v;
      } else {
        println!("A");
        return
      }
    } else if last == 'b' {
      if let Some(v) = sb.pop() {
        last = v;
      } else {
        println!("B");
        return
      }
    } else {
      if let Some(v) = sc.pop() {
        last = v;
      } else {
        println!("C");
        return
      }
    }
  }  
}