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
  let a:Vec<usize> = readvec();
  let (a,b) = (a[0], a[1]);
  
  let mut i = 2usize;
  let mut is_prime = vec![true;1100000];
  let mut is_prime2 = vec![true;b-a+1];
  while i * i <= b {
    if !is_prime[i] {
      i += 1;
      continue
    }
    
    let mut j = i * 2;
    while j * j <= b {
      is_prime[j] = false;
      j += i;
    }

    // start: a 以上の最小の i の倍数
    let mut start = (a+i-1) / i * i;
    if start == i {
      start = i * 2;
    }

    let mut j = start;
    while j <= b {
      is_prime2[j-a] = false;
      j += i;
    }

    i += 1;
  }

  let mut result = 0;
  for i in a..=b {
    if is_prime2[i-a] { result += 1; }
  }
  
  println!("{}", result);
}