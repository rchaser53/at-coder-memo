use std::collections::*;
use std::cmp::Reverse;

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

fn main() {
  let n:usize = readln();
  
  let mut stack = vec![true;n+1];
  stack[0] = false;
  stack[1] = false;

  for i in 2..stack.len() {
    if !stack[i] { continue }
    for j in 2..stack.len() {
      let ni = i * j;
      if stack.len() <= ni { break }
      stack[ni] = false;
    }
  }

  let mut result = 0;
  for i in 0..stack.len()-2 {
    if stack[i] && stack[i+2] { result += 1; }
  }
  
  println!("{}", result);
}