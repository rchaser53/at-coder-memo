/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;


pub fn main(
) {
  input! {
    s:Chars
  }

  let mut result = 1000isize;
  for i in 0..s.len()-2 {
    let mut temp = 0;
    for j in i..i+3 {
      temp += 10isize.pow((i+2-j) as u32) * ((s[j] as u8 - '0' as u8) as isize);
    }
    result = std::cmp::min(result, (753-temp).abs());
  }
  println!("{}", result);
}