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
  let arr:Vec<usize> = readvec();
  let arr:Vec<usize> = readvec();
  let mut map = HashMap::new();
  for v in arr {
    *map.entry(v).or_insert(0) += 1;
  } 
  let mut max = 0;
  for (_, v) in map {
    max = std::cmp::max(max, v);
  }
  println!("{}", max);
}