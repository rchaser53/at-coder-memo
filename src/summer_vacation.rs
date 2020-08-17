use proconio::input;
use std::collections::BinaryHeap;
 
fn main() {
  input! {
    n: usize,
    m: usize,
    mut abs: [(usize,usize);n]
  }
  abs.sort_by(|a, b| a.0.cmp(&b.0));
  abs.reverse();
  
  let mut p_queue = BinaryHeap::new();
  let mut result = 0;
  let mut index = m;
  for index in 1..=m {
    while !abs.is_empty() {
      let last = abs.last().unwrap();
      if last.0 > index {
        break
      }
      p_queue.push(abs.pop().unwrap().1);
    }
    
    if let Some(v) = p_queue.pop() {
      result += v;
    }
  }
  
  println!("{}", result);
}