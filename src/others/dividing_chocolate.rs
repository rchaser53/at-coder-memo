#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    h: usize,
    w: usize,
    k: usize,
    rows: [Chars;h],
  }
  
  let mut min = 1000 * 10;
  let limit = 2usize.pow((h-1) as u32);
  for i in 0..limit {
    let mut blocks = vec![];
    let mut last = 0;
    let mut temp = 0;
    for ii in 0..h-1 {
      if i >> ii & 1 == 1 {
        blocks.push((last, ii));
        last = ii+1;
        temp += 1;
      }
    }
    if last != 0 {
      blocks.push((last, h-1));
    }
    if blocks.is_empty() {
      blocks.push((0,h-1));
    }

    let mut memo = vec![0;blocks.len()];
    let mut last_c = 0;
    for c in 0..w {
      for ii in 0..blocks.len() {
        let (u, d) = blocks[ii];
        for r in u..=d {
          if rows[r][c] == '1' {
            memo[ii] += 1;
          }
        }

        if k < memo[ii] {
          if c == last_c {
            temp = 1000 * 10;
            break
          }
          
          temp += 1;
          for iii in 0..memo.len() {
            memo[iii] = 0;
            let (uu, dd) = blocks[iii];
            for iiii in uu..=dd {
              if rows[iiii][c] == '1' {
                memo[iii] += 1;
              }
            }
          }

          break
        }
      }
      last_c = c;
    }
    min = std::cmp::min(min, temp);
  }
  println!("{}", min);
}
