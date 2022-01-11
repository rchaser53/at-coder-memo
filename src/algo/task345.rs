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

fn helper(n:usize, b:usize) -> usize {
  let mut x = 1;
  let mut set = HashSet::new();
  while x * x <= n {
    if n % x == 0 {
      if b < n/x {
        set.insert(n/x);
      }
      if b < x {
        set.insert(x);
      }
    }
    x += 1;
  }
  set.len()
}

fn main() {
  let a:Vec<usize> = readvec();
  let (a,b) = (a[0], a[1]);
  println!("{}", helper(a-b, b));
}