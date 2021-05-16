#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
 
fn main() {
  input!{
    n:usize,
    s:Chars
  }
  let mut result = VecDeque::new();
  
  for i in 0..n {
    let v = i + 1;
    match s[i] {
      'L' => result.push_front(v),
      'R' => result.push_back(v),
      'A' => {
        if result.len() == 0 {
          println!("ERROR");
        } else {
          println!("{}", result.pop_front().unwrap());
        }
      },
      'B' => {
        if result.len() <= 1 {
          println!("ERROR");
        } else {
          let v = result.pop_front().unwrap();
          println!("{}", result.pop_front().unwrap());
          result.push_front(v);
        }
      },
      'C' => {
        if result.len() <= 2 {
          println!("ERROR");
        } else {
          let v1 = result.pop_front().unwrap();
          let v2 = result.pop_front().unwrap();
          println!("{}", result.pop_front().unwrap());
          result.push_front(v2);
          result.push_front(v1);
        }
      },
      'D' => {
        if result.len() == 0 {
          println!("ERROR");
        } else {
          println!("{}", result.pop_back().unwrap());
        }
      },
      'E' => {
        if result.len() <= 1 {
          println!("ERROR");
        } else {
          let v = result.pop_back().unwrap();
          println!("{}", result.pop_back().unwrap());
          result.push_back(v);
        }
      },
      _ => {
        if result.len() <= 2 {
          println!("ERROR");
        } else {
          let v1 = result.pop_back().unwrap();
          let v2 = result.pop_back().unwrap();
          println!("{}", result.pop_back().unwrap());
          result.push_back(v2);
          result.push_back(v1);
        }
      },
    }
  }
}