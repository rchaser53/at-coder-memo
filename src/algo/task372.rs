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
  let _:usize = readln();
  let s = read_chars();
  let mut map = HashMap::new();
  for c in s {
    *map.entry(c).or_insert(0) += 1;
  }
  let mut result = vec![];
  for _ in 0..*map.get(&'J').unwrap_or(&0) {
    result.push(String::from("J"));
  }
  for _ in 0..*map.get(&'O').unwrap_or(&0) {
    result.push(String::from("O"));
  }
  for _ in 0..*map.get(&'I').unwrap_or(&0) {
    result.push(String::from("I"));
  }
  println!("{}", result.into_iter().collect::<String>());
}