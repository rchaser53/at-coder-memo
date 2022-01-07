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
  let arr:Vec<usize> = readvec();
  let (n,a,b) = (arr[0],arr[1]-1,arr[2]-1);
  let s = read_chars();
  let mut result = vec![];

  for i in 0..a {
    result.push(s[i].to_string());
  }
  let mut temp = vec![];
  for i in a..=b {
    temp.push(s[i].to_string());
  }
  temp.reverse();
  for c in temp {
    result.push(c);
  }
  for i in b+1..n {
    result.push(s[i].to_string());
  }

  println!("{}", result.into_iter().collect::<String>());
}