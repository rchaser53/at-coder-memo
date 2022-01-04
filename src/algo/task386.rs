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
  let _:Vec<usize> = readvec();
  let mut a:Vec<usize> = readvec();
  let mut b:Vec<usize> = readvec();
  a.reverse();
  b.reverse();
  
  let mut result = vec![];
  
  while !a.is_empty() && !b.is_empty() {
    let av = a[a.len()-1];
    let bv = b[b.len()-1];

    if av <= bv {
      a.pop();
      result.push(av);
    } else {
      b.pop();
      result.push(bv);
    }
  }

  for i in (0..a.len()).rev() {
    result.push(a[i]);
  }
  for i in (0..b.len()).rev() {
    result.push(b[i]);
  }
  for v in result {
    println!("{}", v);
  }
}