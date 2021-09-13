use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:Chars,
  }

  let mut memo = vec![0;6];
  for c in s {
    memo[(c as u8 - 'A' as u8) as usize] += 1;
  }

  println!("{}", memo.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}