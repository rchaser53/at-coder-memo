/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    r:usize,
    c:usize,
    rows:[Chars;r]
  }

  let mut result = rows.clone();
  let ir = r as isize;
  let ic = c as isize;
  for i in 0..r {
    for j in 0..c {
      if rows[i][j] != '#' && rows[i][j] != '.' {
        let ii = i as isize;
        let ij = j as isize;
        let num = (rows[i][j] as u8 - '0' as u8) as isize;

        for a in -num..=num {
          for b in -num..=num {
            if num < a.abs() + b.abs() { continue }
            let ni = ii + a;
            let nj = ij + b;

            if ni < 0 || ir <= ni { continue }
            if nj < 0 || ic <= nj { continue }
            let ni = ni as usize;
            let nj = nj as usize;
            result[ni][nj] = '.';
          }
        }
      }
    }
  }

  for arr in result {
    println!("{}", arr.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}