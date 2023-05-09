use std::collections::*;
use std::cmp::*;
use std::io::{stdout, stdin, Write};

fn readln<T: std::str::FromStr>() -> T {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  tmp.trim().parse().ok().unwrap()
}
fn readvec<T: std::str::FromStr>() -> Vec<T> {
  readln::<String>()
      .split_whitespace()
      .map(|x| x.parse().ok().unwrap())
      .collect()
}
fn readchars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

// インタラクティブ interactive
fn main() {
  let n:usize = readln();
  
  let mut left = 0;
  let mut right = n;
  while left + 1 < right {
    let mid = (left + right) / 2;
    println!("? {}", mid);
    stdout().flush();

    let out:usize = readln();
    stdout().flush();

    if out == 1 {
      right = mid;
    } else {
      left = mid;
    }
  }
    
  println!("! {}", left);
  stdout().flush();
  return
}