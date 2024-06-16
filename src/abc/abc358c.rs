/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    ss:[Chars;n]
  }
  

  let mut min = n;
  let limit = 1 << n;
  let need = (1 << m) - 1;
  for i in 0usize..limit {
    let mut temp = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        for x in 0..m {
          if ss[j][x] == 'o' {
            temp |= 1 << x;
          }
        }
      }
    }

    if temp == need {
      min = min.min(i.count_ones() as usize);
    }
  }
  println!("{}", min);
}