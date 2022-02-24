/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::*;
use std::rc::*;
use std::cell::*;

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
  let (n, m) = (a[0], a[1]);

  if n < m {
    println!("0");
  } else if n == m {
    println!("1");
  } else {
    let diff = n - m;
    let b = m - 1;
    let tot = diff + b;

    let mut result = 1;
    for i in 0..diff {
      result *= tot - i;
    }
    for i in 1..=diff {
      result /= i;
    }

    println!("{}", result);
  }
  
}