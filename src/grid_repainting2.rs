#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashSet;

fn main() {
  input! {
    h: usize,
    w: usize,
    s: [Chars;h],
  }
  
  for r in 0..h {
    for c in 0..w {
      if s[r][c] == '.' { continue }
      let mut count = 0;
      if 0 < r && s[r-1][c] == '#' {
        count += 1;
      }
      
      if 0 < c && s[r][c-1] == '#' {
        count += 1;
      }
      
      if r < h-1 && s[r+1][c] == '#' {
        count += 1;
      }
      
      if c < w-1 && s[r][c+1] == '#' {
        count += 1;
      }
      
      if count == 0 {
        println!("No");
        return;
      }
    }
  }
  println!("Yes");
}