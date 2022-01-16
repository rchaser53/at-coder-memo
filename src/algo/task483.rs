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
fn readchars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

fn main() {
  let n:usize = readln();
  let mut i = 1;
  let mut set = HashSet::new();
  while i * i <= n {
    if n % i == 0 {
      set.insert(n/i);
      set.insert(i);
    }
    i += 1;
  }

  let mut arr = set.clone().into_iter().collect::<Vec<usize>>();
  arr.sort();
  let max = arr[arr.len()-1];
  let mut black = HashSet::new();  
  let mut result = 1;
  for i in 1..arr.len() {
    let v1 = arr[i];
    if black.contains(&v1) { continue }
    result += 1;
    for j in i..arr.len() {
      let v2 = v1 * arr[j];
      if max < v2 { break }
      black.insert(v2);
    }
  }

  println!("{}", result);
}