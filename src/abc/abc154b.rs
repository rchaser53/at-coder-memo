/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    mut s:Chars,
  }

  for i in 0..s.len() {
    s[i] = 'x';
  }

  println!("{}", s.iter().map(|v| v.to_string()).collect::<String>());
}
