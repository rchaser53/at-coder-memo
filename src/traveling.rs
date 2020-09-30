#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
 
fn main() {
  input!{
    n: usize,
    vals: [(isize,isize,isize);n],
  }
  let mut last = (0, 0, 0);
  for i in 0..n {
    let val = vals[i];
    let diff_t = val.0 - last.0;
    let need_val = (val.1 - last.1).abs() + (val.2 - last.2).abs();
    if diff_t < need_val {
      println!("No");
      return
    } else if (diff_t - need_val) % 2 == 1 {
      println!("No");
      return
    }
    last = val;
  }
  
  println!("Yes");
}