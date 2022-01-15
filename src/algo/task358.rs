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

fn create_primes(n:usize) -> Vec<usize> {
  let mut memo = vec![true;n+1];
  memo[0] = false;
  memo[1] = false;
  for i in 2..=n {
    if !memo[i] { continue }
    for j in 2.. {
      let ni = i * j;
      if n < ni { break }
      memo[ni] = false;
    }
  }

  let mut primes = vec![];
  for i in 0..memo.len() {
    let v = memo[i];
    if v {
      primes.push(i);
    }
  }
  primes
}


fn main() {
  let n:usize = readln();
  let limit = 10usize.pow(5);
  let primes = create_primes(limit+1);
  let mut set = HashSet::new();
  for i in primes {
    if i * i <= n {
      set.insert(i*i);
    }
  }

  println!("{}", set.len());
}