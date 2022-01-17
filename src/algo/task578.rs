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
  let vals:Vec<usize> = readvec();
  let (n, m) = (vals[0], vals[1]);

  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= m {
    if m % i == 0 {
      set.insert(i);
      set.insert(m/i);
    }
    i += 1;
  }

  let mut arr = set.into_iter().collect::<Vec<usize>>();
  arr.sort();

  let mut result = 0usize;
  for bv in arr {
    let mut v = 1;
    let base = bv;
    let mut success = true;
    for _ in 0..n {
      v *= base;
      if m % v != 0 {
        success = false;
        break
      }
    }
    if success {
      result += 1;
    }
  }

  println!("{}", result);
}