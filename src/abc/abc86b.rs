#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    a:String,
    b:String
  }
  
  let mut set = HashSet::new();
  for i in 2..=1000 {
    set.insert(i*i);
  }

  let v = format!("{}{}", a, b);
  let v = v.parse::<usize>().unwrap();
  if set.contains(&v) {
    println!("Yes");
  } else {
    println!("No");
  }
}