/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    mut vals:[usize;n]
  }

  let mut result = vec![false;k+1];
  vals.sort();
  for i in 0..k {
    if result[i] { continue }
    for &v in &vals {
      let ni = i + v;
      if k < ni { break }
      result[ni] = true;
    }
  }
  
  if result[k] {
    println!("First");
  } else {
    println!("Second");
  }
}