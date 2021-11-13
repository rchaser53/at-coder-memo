use proconio::input;
use std::collections::HashMap;
 
fn main() {
  input! {
    N: usize,
    K: usize,
    mut A: [usize;N]
  }
 
  let mut current = 1;
  let mut pattern = vec![current];
  let mut memo: HashMap<usize, usize> = HashMap::new();
  while pattern.len() < K {
    let next = A[current-1];
    
    if current == next {
      println!("{}", next);
      return
    }
    
    if let Some(_) = memo.get_mut(&next) {
      let mut loop_start_index = 0;
      let pattern_len = pattern.len();
      for (i, v) in pattern.iter().enumerate() {
        if v == &next {
          loop_start_index = i;
            break;
        }
      }
 
      let loop_count = pattern.len() - loop_start_index;
      let left = K - loop_start_index;
      println!("{}", pattern[loop_start_index + (left % loop_count)]);
      return;
    } else {
      memo.insert(current, next);
      pattern.push(next);
    }
    current = next;
  }
 
  println!("{}", A[current-1]);
}