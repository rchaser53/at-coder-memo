#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn culc(
  mut deque: VecDeque<isize>,
  mut count_que: VecDeque<isize>,
  mut index: usize
) -> isize {
  while !deque.is_empty() {
    let left = count_que[0];
    let right = count_que[count_que.len()-1];
    
    let large = std::cmp::max(
      (left - deque[deque.len()-1]).abs(),
      (right - deque[deque.len()-1]).abs()
    );
    let small = std::cmp::max(
      (left - deque[0]).abs(),
      (right - deque[0]).abs()
    );
    
    let v = if small < large {
      deque.pop_back().unwrap()
    } else {
      deque.pop_front().unwrap()
    };
        
    if (left - v).abs() < (right - v).abs() {
      count_que.push_back(v);    
    } else {
      count_que.push_front(v);
    }
  }

  let mut result = 0;
  let mut last = count_que.pop_front().unwrap();
  for v in count_que {
    let add_val = (last - v).abs();
    result += add_val;
    last = v;
  }
  result
}

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [isize;n]
  }
  
  vals.sort();
  let mut deque_a: VecDeque<isize> = vals.into_iter().collect();
  let mut deque_b = deque_a.clone();
  let mut count_que_a: VecDeque<isize> = VecDeque::new();
  let mut count_que_b: VecDeque<isize> = VecDeque::new();
  
  count_que_a.push_back(deque_a.pop_front().unwrap());
  count_que_b.push_back(deque_b.pop_back().unwrap());
  let a = culc(deque_a, count_que_a, 0);
  let b = culc(deque_b, count_que_b, 1);
  
  println!("{}", std::cmp::max(a, b));
}