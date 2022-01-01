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
  let dict:Vec<usize> = readvec();
  let n = dict[0];
  let s = read_chars();
  let mut dict = vec!['I', 'O', 'I'];
  let mut i = 0;
  while !dict.is_empty() && i < n {
    if dict[dict.len()-1] == s[i] {
      dict.pop();
    }
    i += 1;
  }

  if dict.is_empty() {
    println!("Yes");
  } else {
    println!("No");
  }
}