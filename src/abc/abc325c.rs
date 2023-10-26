/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }

  let mut count = 0;
  let p1 = [-1,0,1];
  let p2 = [-1,0,1];
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == '#' {
        let mut stack = vec![(i as isize, j as isize)];
        rows[i][j] = '.';
        count += 1;

        while !stack.is_empty() {
          let mut new_stack = vec![];
          while let Some((ci,cj)) = stack.pop() {
            for &ai in &p1 {
              let ni = ci + ai;
              for &aj in &p2 {
                if ai == 0 && aj == 0 { continue }
                let nj = cj + aj;

                if 0 <= ni && ni < h as isize && 0 <= nj && nj < w as isize
                  && rows[ni as usize][nj as usize] == '#' {
                    rows[ni as usize][nj as usize] = '.';
                    new_stack.push((ni,nj));
                }
              }
            }
          }
          stack = new_stack;
        }
      }
    }
  }
  println!("{}", count);
}