/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn main() {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut memo = vec![true;51];
  memo[0] = false;
  memo[1] = false;
  for i in 2..=50 {
    if !memo[i] {
      continue
    }
    for j in 2.. {
      let ni = i * j;
      if 50 < ni { break }
      memo[ni] = false;
    }
  }
  
  let mut dict = vec![];
  for i in 2..=50 {
    let v = memo[i];
    if v {
      dict.push(i);
    }
  }

  let n = dict.len();
  let limit = 1 << n;
  let mut min = usize::max_value();
  for i in 1..limit {
    let mut temp = 1;
    for j in 0..n {
      if i >> j & 1 == 1 {
        temp *= dict[j];
      }
    }

    let mut success = true;
    for &v in &vals {
      if gcd(v, temp) == 1 {
        success = false;
        break
      }
    }

    if success {
      min = std::cmp::min(min, temp);
    }
  }
  
  println!("{}", min);
}