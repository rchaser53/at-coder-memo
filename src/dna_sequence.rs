#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    s: Chars
  }
  
  let mut total = 0;
  for i in 0..s.len() {
    let mut at = 0;
    let mut gc = 0;
    for ii in i..s.len() {
      match s[ii] {
        'A' => { at += 1; },
        'T' => { at -= 1; },
        'G' => { gc += 1; },
        'C' => { gc -= 1; },
         _ => {}
      }
      
      if at == 0 && gc == 0 {
        total += 1;
      }
    }
  }
  println!("{}", total);
}
