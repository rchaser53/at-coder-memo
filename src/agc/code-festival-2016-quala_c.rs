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
    mut s: Chars,
    mut k: isize
  }

  let n = s.len();
  for i in 0..n {
    if s[i] == 'a' { continue }
    let v = (s[i] as u8 - 'a' as u8) as isize;
    let need = 26 - v;
    if need <= k {
      s[i] = 'a';
      k -= need;
    }
  }

  let v = (s[n-1] as u8 - 'a' as u8) as isize + k;
  s[n-1] = ((v % 26) as u8 + 'a' as u8) as char;
  println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
}