/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    kk:usize,
    s:[Chars;n]
  }

  let mut max = 1;
  for c in 0..10 {
    for i in 0..n {
      for j in 0..m {
        for k in 1.. {
          let hm = i+k;
          let wm = j+k;
          if n < hm || m < wm {
            break
          }
          
          let mut temp = 0;
          for ri in i..hm {
            for rj in j..wm {
              let cc = (s[ri][rj] as u8 - '0' as u8) as usize;
              if cc != c {
                temp += 1;
              }
            }
          }

          if temp <= kk {
            max = max.max(k);
          }
        }
      }
    }
  }

  println!("{}", max);
}