/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    s:String,
    t:Chars,
  }

  let n = s.len();
  for i in 0..=n {
    let mut temp = vec![];
    for j in 0..n {
      temp.push(t[(i+j)%n]);
    }

    let ts = temp.iter().map(|v|v.to_string()).collect::<String>();
    if ts == s {
      println!("Yes");
      return
    }
  }
  println!("No");
}
