/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    a:[i128;n]
  }

  let mut now = 1;
  for ai in a {
    now *= ai;
    if now.to_string().len() > k {
      now = 1;
    }
  }

  println!("{}", now);  
}