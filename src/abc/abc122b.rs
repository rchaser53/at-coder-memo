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
    s: Chars
  }

  let whitelist = vec!['A','C','G','T'].into_iter().collect::<HashSet<char>>();
  let mut result = 0;
  let mut temp = 0;
  for c in s {
    if whitelist.contains(&c) {
      temp += 1;
      result = std::cmp::max(result, temp);
    } else {
      result = std::cmp::max(result, temp);
      temp = 0;
    }
  }
  println!("{}", result);
}
