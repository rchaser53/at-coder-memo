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
  let (g,m) = (a[0],a[1]);
  if m % g != 0 {
    println!("-1");
    return
  }

  let mut left = m / g;
  let mut i = 2;
  let mut map = HashMap::new();
  while i * i <= left {
    if left % i == 0 {
      *map.entry(i).or_insert(0) += 1;
      left /= i;
    } else {
      i += 1;
    }
  }

  if 1 < left {
    *map.entry(left).or_insert(0) += 1;
  }

  let mut min = m + g;

  let mut memo = vec![];
  for (key, val) in map {
    memo.push(key.pow(val as u32));
  }

  let n = memo.len();
  let limit = 1 << n;
  for i in 0..limit {
    let mut left = g;
    let mut right = g;
    for j in 0..n {
      if i >> j & 1 == 1 {
        left *= memo[j];
      } else {
        right *= memo[j];
      }
    }
    min = std::cmp::min(min, left + right);
  }

  println!("{}", min);
}