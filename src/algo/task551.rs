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
  let n:usize = readln();
  let mut vals:Vec<usize> = readvec();
  vals.sort();
  let diff = vals[1] - vals[0];

  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= diff {
    if diff % i == 0 {
      set.insert(i);
      set.insert(diff/i);
    }
    i += 1;
  }

  let mut result = 0;
  for v in set {
    let rv = vals[0] % v;
    let mut success = true;
    for &val in &vals {
      if val % v != rv {
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