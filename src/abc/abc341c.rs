/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    n:usize,
    t:Chars,
    rows:[Chars;h]
  }

  let mut result = 0;
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == '.' {
        let mut ci = i;
        let mut cj = j;
        let mut success = true;
        for &c in t.iter().rev() {
          if c == 'L' && rows[ci][cj+1] == '.' {
            cj += 1;
          } else if c == 'R' && rows[ci][cj-1] == '.' {
            cj -= 1;
          } else if c == 'U' && rows[ci+1][cj] == '.' {
            ci += 1;
          } else if c == 'D' && rows[ci-1][cj] == '.' {
            ci -= 1;
          } else {
            success = false;
            break;
          }
        }

        if success {
          result += 1;
        }
      }
    }
  }
  
  println!("{}", result);
}