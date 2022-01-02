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
  let dict:Vec<usize> = readvec();

  let mut max = 0;
  let mut ti = 0;
  for i in 0..n {
    if max < dict[i] {
      ti = i;
      max = dict[i];
    }
  }
  
  let mut result = 0;
  for i in 0..ti {
    result += dict[i];
  }
  println!("{}", result);

  let mut result = 0;
  for i in ti+1..n {
    result += dict[i];
  }
  println!("{}", result);

}