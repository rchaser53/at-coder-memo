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
fn read_chars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

fn main() {
  let n:usize = readln();
  let vals:Vec<usize> = readvec();
  
  let mut result = 1;
  let mut stack = vec![];

  for v in vals {
    if stack.is_empty() {
      stack.push(v);
    } else {
      if stack[stack.len()-1] > v {
        result = std::cmp::max(result, stack.len());
        stack = vec![v];
      } else {
        stack.push(v);
      }
    }
  }
  result = std::cmp::max(result, stack.len());
  println!("{}", result);
}