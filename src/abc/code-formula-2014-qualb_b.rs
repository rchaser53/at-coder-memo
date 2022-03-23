/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    mut s:Chars
  }
  s.reverse();
  
  let mut a = 0;
  let mut b = 0;
  for i in 0..s.len() {
    let v = s[i].to_string().parse::<usize>().unwrap();
    if i % 2 == 0 {
      a += v;
    } else {
      b += v;
    }
  }
  println!("{} {}", b, a);
}