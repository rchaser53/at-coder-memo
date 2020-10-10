#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    mut s: Chars
  }
  
  let mut temp_stack: Vec<char> = vec![];
  for i in 0..n {
    if temp_stack.is_empty() {
      temp_stack.push(s[i]);
    } else if temp_stack.last().unwrap() == &'('
        && s[i] == ')' {
      temp_stack.pop();
    } else {
      temp_stack.push(s[i]);
    }
  }
  
  let mut lc = 0;
  let mut rc = 0;
  for c in temp_stack {
    if c == '(' {
      lc += 1;
    } else {
      rc += 1;
    }
  }
  
  let mut result: Vec<String> = s.into_iter().map(|c| c.to_string()).collect();
  for i in 0..lc {
    result.push(String::from(")"));
  }
  for i in 0..rc {
    result.insert(0, String::from("("));
  }
  println!("{}", result.into_iter().collect::<String>());
}