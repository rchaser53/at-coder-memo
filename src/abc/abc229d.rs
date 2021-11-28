/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    s:Chars,
    k:usize
  }

  if k == 0 {
    let mut stack = vec![];
    let mut max = 0;

    for c in s {
      if stack.is_empty() {
        stack.push(c);
      } else if stack[0] != c {
        let len = stack.len();
        if stack[0] == 'X' {
          max = std::cmp::max(len, max);
        }
        stack = vec![c];
      } else {
        stack.push(c);
      }
    }
    if stack[0] == 'X' {
      max = std::cmp::max(max, stack.len());
    }
    println!("{}", max);
    return
  }

  let n = s.len();
  let mut max = 0usize;
  let mut que: VecDeque<usize> = VecDeque::new();
  let mut set = HashSet::new();
  for i in 0..n {
    let c = s[i];
    if c == '.' {
      if set.len() == k {
        max = std::cmp::max(max, que[que.len()-1] - que[0] + 1);
        while let Some(ci) = que.pop_front() {
          if set.contains(&ci) {
            set.remove(&ci);
            set.insert(i);
            que.push_back(i);
            break
          }
        }
      } else {
        set.insert(i);
        que.push_back(i);
      }
    } else {
      que.push_back(i);
    }
  }
  max = std::cmp::max(max, que[que.len()-1] - que[0] + 1);
  

  println!("{}", max);
}