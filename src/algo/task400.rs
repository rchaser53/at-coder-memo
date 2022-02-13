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
  let (n,a,b,c,d) = (a[0],a[1],a[2],a[3],a[4]);

  let mut av = n / a;
  if n % a != 0 {
    av += 1;
  }
  let mut cv = n / c;
  if n % c != 0 {
    cv += 1;
  }

  println!("{}", std::cmp::min(av * b, cv * d));
}