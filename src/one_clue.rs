use proconio::input;
use std::collections::VecDeque;
 
fn main() {
  input! {
    k: isize,
    x: isize,
  }
  
  let mut result = VecDeque::new();
  result.push_back(x);
  
  for i in 1..k {
    result.push_front(x-i);
    result.push_back(x+i);
  }
  
  println!(
    "{}",
    result
      .into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .join(" ")
  );
}