/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
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
  
  let mut stack = vec![true;10usize.pow(4)+10];
  
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

  let mut set = HashSet::new();
  for i in 0..stack.len() {
    if stack[i] { set.insert(i); }
  }
  let mut result = 1_000_000;
  let set = set.into_iter().collect::<Vec<usize>>();
  for &v in &set {
    for &nv in &set {
      if v + nv == n {
        result = result.min(v).min(nv);
      }
    }
  }
  
  if result == 1_000_000 {
    println!("-1");
  } else {
    println!("{}", result);
  }
}