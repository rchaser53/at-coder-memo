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
  let dict:Vec<usize> = readvec();
  let mut map = HashMap::new();
  for v in dict {
    *map.entry(v).or_insert(0) += 1;
  }
  let mut a = map.into_iter().collect::<Vec<(usize,usize)>>();
  a.sort_by(|a,b| a.1.cmp(&b.1));
  println!("{}", a[a.len()-1].0);
}