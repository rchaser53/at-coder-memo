use proconio::input;
use proconio::marker::*;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    chars: String
  }
  
  let mut left = vec![0;26];
  let mut right = vec![0;26];
  for v in chars.as_bytes().iter() {
    let i = (v - 97u8) as usize;
    right[i] += 1;
  }
  
  let mut max = 0;
  for v in chars.as_bytes().iter() {
    let i = (v - 97u8) as usize;
    right[i] -= 1;
    left[i] += 1;
    
    let mut temp = 0;
    for ii in 0..26 {
      if 0 < right[ii] && 0 < left[ii] {
        temp += 1;
      }
    }
    max = std::cmp::max(max, temp);
  }
   
  println!("{}", max);
}