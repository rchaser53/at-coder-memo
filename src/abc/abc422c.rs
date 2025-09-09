/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:usize,
    abc:[(usize,usize,usize);t]
  }

  for (a, b, c) in abc {
    println!("{}", a.min(c).min((a+b+c)/3));
  }
}