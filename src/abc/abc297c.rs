/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }
  
  for i in 0..h {
    for j in 0..w-1 {
      if rows[i][j] == 'T' && rows[i][j+1] == 'T' {
        rows[i][j] = 'P';
        rows[i][j+1] = 'C';
      }
    }
  }

  for row in rows {
    println!("{}", row.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}