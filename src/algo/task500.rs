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
  let a:Vec<usize> = readvec();
  let (a,b,r,s) = (a[0],a[1],a[2],a[3]);
  let ra = a - r;
  let rb = b - s;
  let v = gcd(ra, rb);

  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= v {
    if v % i == 0 {
      set.insert(i);
      set.insert(v/i);
    }
    i += 1;
  }

  let mut min = usize::max_value();
  for v in set {
    if a % v == r && b % v == s {
      min = std::cmp::min(v, min);
    }
  }

  if min == usize::max_value() {
    println!("-1");
  } else {
    println!("{}", min);
  }
}