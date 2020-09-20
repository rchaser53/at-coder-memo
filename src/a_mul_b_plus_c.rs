use proconio::input;
use std::collections::{HashMap, VecDeque};
 
fn main() {
  input! {
    n: usize,
  }
  let mut result = n-1;
  
  for i in 2..=n {
    for ii in 1..=n {
      if n - 1 < i * ii { break }
      result += 1;
    }
  }
  println!("{}", result);
}