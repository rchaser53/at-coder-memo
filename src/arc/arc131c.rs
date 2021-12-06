/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }

  let diff = ['1','2','3','4','5'];

  for i in 0..h {
    for j in 0..w {
      if rows[i][j] != '.' { continue }
      let mut set = HashSet::new();
      if 0 < i {
        set.insert(rows[i-1][j]);
      }
      if i < h-1 {
        set.insert(rows[i+1][j]);
      }
      if 0 < j {
        set.insert(rows[i][j-1]);
      }
      if j < w-1 {
        set.insert(rows[i][j+1]);
      }

      for &c in &diff {
        if !set.contains(&c) {
          rows[i][j] = c;
        }
      }
    }
  }

  for row in rows {
    println!("{}", row.iter().map(|v| v.to_string()).collect::<String>());
  }
}