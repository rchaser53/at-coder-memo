#![allow(unused_imports)]
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::VecDeque;
 
fn main() {
  input! {
    n: usize,
    vals: [usize;n],
  }
  
  let mut result: VecDeque<usize> = vec![].into_iter().collect();
  for (i, v) in vals.into_iter().enumerate() {
    if i % 2 == 0 {
      result.push_back(v);
    } else {
      result.push_front(v);
    }
  }
  
  let mut result:Vec<usize> = result.into_iter().collect();
  if n % 2 == 1 {
    result.reverse();
  }
  
  println!("{}",
    result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ")
  );
}