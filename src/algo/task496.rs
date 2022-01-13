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

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn main() {
  let n:usize = readln();
  let vals:Vec<usize> = readvec();

  let mut a = vals[0];

  for i in 1..n {
    a = gcd(a, vals[i]);
  }

  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= a {
    if a % i == 0 {
      set.insert(a/i);
      set.insert(i);
    }
    i += 1;
  }

  println!("{}", set.len());
}