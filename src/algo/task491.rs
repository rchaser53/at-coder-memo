/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::*;

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
  let a:Vec<usize> = readvec();
  let (n, mut k) = (a[0],a[1]);
  let mut vals = vec![];
  for _ in 0..n {
    let a:Vec<usize> = readvec();
    let (n,k) = (a[0],a[1]);
    vals.push((n,k));
  }
  vals.sort();
  
  let mut result = 0usize;
  for (v, num) in vals {
    if k <= num {
      result += k * v;
      break
    }
    result += num * v;
    k -= num;
  }

  println!("{}", result);
}