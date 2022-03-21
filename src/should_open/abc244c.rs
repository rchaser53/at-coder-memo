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
  
  let mut set = HashSet::new();
  let limit = 2 * n + 1;
  set.insert(1);
  println!("1");
  stdout().flush();
 
  for _ in 0..n {
    let a:usize = readln();
 
    set.insert(a);
    for i in 1..=limit {
      if !set.contains(&i) {
        set.insert(i);
        println!("{}", i);
        stdout().flush();
        break
      }
    }
  }
  return
}