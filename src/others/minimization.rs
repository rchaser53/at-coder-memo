use proconio::input;
use std::collections::{HashMap, VecDeque};
 
fn main() {
  input! {
    n: usize,
    k: usize,
    mut vals: [usize;n]
  }
  
  if n <= k {
    println!("1");
  } else {
    let mut val = (n - k) / (k - 1) + 1;
    if (n - k) % (k - 1) > 0 {
      val += 1;
    }
    println!("{}", val); 
  }
}