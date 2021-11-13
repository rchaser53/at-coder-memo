use proconio::input;
use std::collections::BinaryHeap;
 
fn main() {
  input! {
    n: usize,
    hs: [usize;n]
  }
  
  let mut index = hs.len() - 1;
  let mut last = hs[index];  
  while 0 < index {
    let val = hs[index];
    if last + 1 < val {
      println!("No");
      return
    } else if last + 1 == val {
      last = val - 1;
    } else {
      last = val;
    }
    index -= 1;
  }
  
  if last + 1 < hs[0] {
    println!("No");
  } else {
    println!("Yes");
  }
}