use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input!{
    s:Chars,
  }

  let n = s.len();

  let mut count = 0;
  for c in s {
    if c == 'o' {
      count += 1;
    }
  }
  count += 15 - n;
  if 7 < count {
    println!("YES");
  } else {
    println!("NO");
  }
}