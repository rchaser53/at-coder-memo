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
  let mut n:usize = readln();
  
  if n == 1 {
    println!("1");
    return
  }

  let mut map = HashMap::new();
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      *map.entry(i).or_insert(0) += 1;
      n /= i;
    } else {
      i += 1;
    }
  }

  if n != 1 {
    *map.entry(n).or_insert(0) += 1;
  }

  let mut result = 1;
  for (key, val) in map {
    if val % 2 != 0 {
      result *= key;
    }
  }
  println!("{}", result);
}