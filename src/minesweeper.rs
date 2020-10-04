#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
 
fn main() {
  input!{
    h: usize,
    w: usize,
    rows: [Chars;h]
  }
  
  for r in 0..h {
    let mut temp = String::from("");
    for c in 0..w {
      if rows[r][c] == '#' {
        temp = format!("{}#", &temp);
        continue
      }
      
      let mut count = 0;
      if 0 < r {
        if rows[r-1][c] == '#' { count += 1; }
        if 0 < c && rows[r-1][c-1] == '#' { count += 1; }
        if c < w-1 && rows[r-1][c+1] == '#' { count += 1; }
      }
      if 0 < c && rows[r][c-1] == '#' { count += 1; }
      if c < w-1 && rows[r][c+1] == '#' { count += 1; }
      if r < h-1 {
        if rows[r+1][c] == '#' { count += 1; }
        if 0 < c && rows[r+1][c-1] == '#' { count += 1; }
        if c < w-1 && rows[r+1][c+1] == '#' { count += 1; }
      }
      temp = format!("{}{}", temp, count);
    }
    println!("{}", temp);
  }
}
