use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    s:Chars,
    t:Chars
  }

  let mut ci = 0;
  let mut result = 0;
  for i in 0..n {
    let c = if ci != 0 && i < ci {
      '0'
    } else {
      s[i]
    };
    if c != t[i] {
      ci = std::cmp::max(ci, i+1);
      while ci < s.len() && s[ci] != '1' {
        ci += 1;
      }
      if ci == s.len() {
        println!("-1");
        return
      }
      result += ci - i;
      ci += 1;
    }
  }
  println!("{}", result);
}