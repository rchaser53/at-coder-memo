#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    vals: Chars,
  }
  
  let str_vals: Vec<String> = vals
    .into_iter()
    .map(|v| v.to_string())
    .collect();
  let vals: Vec<usize> = str_vals
    .clone()
    .into_iter()
    .map(|v| v.parse::<usize>().unwrap())
    .collect();
  
  let limit = 2usize.pow(3);
  for i in 0..limit {
    let mut start = vals[0];
    let mut opes: Vec<String> = vec![];
    if i & 1 == 1 {
      start += vals[1];
      opes.push(String::from("+"));
    } else {
      start -= vals[1];
      opes.push(String::from("-"));
    }
    
    if i >> 1 & 1 == 1 {
      start += vals[2];
      opes.push(String::from("+"));
    } else {
      start -= vals[2];
      opes.push(String::from("-"));
    }
    
    if i >> 2 & 1 == 1 {
      start += vals[3];
      opes.push(String::from("+"));
    } else {
      start -= vals[3];
      opes.push(String::from("-"));
    }
    
    if start == 7 {
      let mut result = str_vals[0].clone();
      for i in 0..3 {
        result = format!("{}{}{}", result, opes[i].clone(), str_vals[i+1].clone());
      }
      println!("{}=7", result);
      return
    }
  }
}
