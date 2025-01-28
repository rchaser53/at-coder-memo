/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let mut lt = (h-1,w-1);
  let mut rb = (0,0);
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == '#' {
        lt.0 = lt.0.min(i);
        rb.0 = rb.0.max(i);
        lt.1 = lt.1.min(j);
        rb.1 = rb.1.max(j);
      }
    }
  }

  for i in lt.0..=rb.0 {
    for j in lt.1..=rb.1 {
      if rows[i][j] == '.' {
        println!("No");
        return
      }
    }
  }
  println!("Yes");
}