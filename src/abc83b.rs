#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap,VecDeque};

fn main() {
  input!{
    n:usize,
    a:usize,
    b:usize
  }
  
  let mut count = 0;
  for i in 1..=n {
    let v = i.to_string().chars().collect::<Vec<char>>();
    let mut temp = 0;
    for vv in v {
      temp += vv.to_string().parse::<usize>().unwrap();
    }

    if a <= temp && temp <= b {
      count += i;
    }
  }
  
  println!("{}", count);
}