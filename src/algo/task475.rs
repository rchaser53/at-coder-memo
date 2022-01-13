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

fn gcd(a: i128, b: i128) -> i128 {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn lcm(a: i128, b: i128) -> i128 {
  a * b / gcd(a, b)
}

fn main() {
  let n:Vec<i128> = readvec();
  let (a,b) = (n[0], n[1]);
  
  println!("{}", lcm(a,b));
}