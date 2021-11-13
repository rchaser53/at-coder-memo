#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, HashSet};

fn main() {
  input! {
    n: usize,
    reds: [(isize, isize);n],
    blues: [(isize, isize);n]
  }
  
  let mut mixed = vec![];
  for i in 0..n {
    mixed.push((reds[i].0, reds[i].1, false));
    mixed.push((blues[i].0, blues[i].1, true));
  }
  mixed.sort_by(|a, b| a.0.cmp(&b.0));
  mixed.reverse();
    
  let mut result = 0;
  let mut red_stack = vec![];
  while let Some((_, y, is_blue)) = mixed.pop() {
    if is_blue && red_stack.is_empty() {
      continue
    } else if !is_blue {
      red_stack.push(y);
      continue
    }
    
    let mut index = None;
    for i in 0..red_stack.len() {
      if y <= red_stack[i] { continue }
      if let Some(ii) = index {
        if red_stack[ii] < red_stack[i] {
          index = Some(i); 
        }
      } else {
        index = Some(i);
      }
    }
        
    if let Some(i) = index {
      red_stack.remove(i);
      result += 1;
    }
  }
  
  println!("{}", result);
}