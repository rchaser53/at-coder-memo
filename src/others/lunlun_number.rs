use proconio::input;
use std::collections::VecDeque;
 
fn main() {
  input! { K: u32 }
  let mut queue:VecDeque<u32> = vec![1,2,3,4,5,6,7,8,9].into_iter().collect();
  let mut answer = 0;
 
  for _ in 0..K {
    let val = queue.pop_front().unwrap();
    answer = val;
    
    let next = val * 10 + val % 10;
    if next % 10 != 0 {
      queue.push_back(next - 1);
    }
    queue.push_back(next);
    if next % 10 != 9 {
      queue.push_back(next + 1);
    }
  }
  println!("{}", answer);
}
