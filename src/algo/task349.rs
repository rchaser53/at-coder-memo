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

fn helper(a:usize) -> HashSet<usize> {
  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= a {
    if a % i == 0 {
      set.insert(i);
      set.insert(a/i);
    }
    i += 1;
  }

  set
}

fn main() {
  let a:usize = readln();

  let set = helper(a);

  let mut tot = 0;
  for v in set {
    if a == v { continue }
    tot += v;
  }

  if a == tot {
    println!("Yes");
  } else {
    println!("No");
  }
}