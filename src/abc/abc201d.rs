/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      h:usize,
      w:usize,
      rows:[Chars;h]
    }

    let mut memo = vec![vec![0;w];h];
    for i in (0..h-1).rev() {
      memo[i][w-1] = memo[i+1][w-1];
      let mut v = 1;
      if (i+w-1) % 2 == 1 {
        v = -1;
      }
      if rows[i+1][w-1] == '+' {
        v *= 1;
      } else {
        v *= -1;
      }
      memo[i][w-1] += v;
    }
    for i in (0..w-1).rev() {
      memo[h-1][i] = memo[h-1][i+1];
      let mut v = 1;
      if (i+h-1) % 2 == 1 {
        v = -1;
      }
      if rows[h-1][i+1] == '+' {
        v *= 1;
      } else {
        v *= -1;
      }
      memo[h-1][i] += v;
    }

    for i in (0..h-1).rev() {
      for j in (0..w-1).rev() {
        let mut p = 1;
        if (i+j) % 2 == 1 {
          p = -1;
        }
        let vi = memo[i+1][j] + p * if rows[i+1][j] == '+' {
          1
        } else {
          -1
        };
        let vj = memo[i][j+1] + p * if rows[i][j+1] == '+' {
          1
        } else {
          -1
        };
        memo[i][j] = if (i+j) % 2 == 0 {
          vi.max(vj)
        } else {
          vi.min(vj)
        };
      }
    }
    if memo[0][0] == 0 {
      println!("Draw");
    } else if 0 < memo[0][0] {
      println!("Takahashi");
    } else {
      println!("Aoki");
    }
}
