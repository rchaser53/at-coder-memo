/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;


fn main() {
  input! {
    n:usize,
    rows:[Chars;n]
  }

  let rows = rows.into_iter().map(|arr| {
    arr.into_iter().map(|v| (v as u8 - '0' as u8) as usize).collect::<Vec<usize>>()
  }).collect::<Vec<Vec<usize>>>();

  let mut result = usize::max_value();
  for target_val in 0..10 {
    
    let mut base = 0;
    for i in 0..10 {
      let mut temp = 0;
      for j in 0..n {
        if rows[j][i] == target_val {
          temp += 1;
        }
      }
      if 0 < temp {
        base = std::cmp::max(base, (temp - 1) * 10 + i);
      }
    }
    result = std::cmp::min(result, base);
  }

  println!("{}", result);
}