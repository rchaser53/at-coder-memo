/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    mut t:Chars
  }
  for i in 0..t.len() {
    if t[i] == '?' {
      t[i] = 'D';
    }
  }

  println!("{}", t.into_iter().map(|v| v.to_string()).collect::<String>());
}