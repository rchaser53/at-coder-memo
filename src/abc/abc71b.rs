/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 998244353;
pub fn main(
) {
  input! {
    s:Chars
  }

  let mut memo = vec![true;26];
  for i in 0..s.len() {
    let c = (s[i] as u8 - 'a' as u8) as usize;
    memo[c] = false;
  }

  for i in 0..26 {
    if memo[i] {
      println!("{}", (i as u8 + 'a' as u8) as char);
      return
    }
  }
  println!("None");
}
