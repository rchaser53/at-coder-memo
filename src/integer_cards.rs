use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
  input! {
    n: usize,
    m: usize,
    values: [usize;n],
    mut changes: [(usize, usize);m]
  }  
  changes.sort_by(|a,b| a.1.cmp(&b.1));
  changes.reverse();
  
  let mut p_queue = BinaryHeap::new();
  for v in values {
    p_queue.push(Reverse(v));
  }
  
  let mut last_val = usize::max_value();
  for (i, v) in changes {
    for ii in 0..i {
      let val = p_queue.pop().unwrap();
      if v <= val.0 {
        p_queue.push(val);
        break;
      }
      p_queue.push(Reverse(v));
    }
    last_val = v;
  }
  
  println!("{}", p_queue.iter().map(|v| v.0).sum::<usize>());
}