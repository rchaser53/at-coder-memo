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
  let a:Vec<usize> = readvec();
  let (n) = (a[0]);
  let vals:Vec<usize> = readvec();
  let mut vals = vals.into_iter().enumerate().collect::<Vec<(usize,usize)>>();
  vals.sort_by(|a,b| a.1.cmp(&b.1));
  vals.reverse();
  
  let mut set = HashSet::new();
  let mut result = vec![0;n];

  for (i, v) in vals {
    set.insert(v);
    result[i] = set.len() - 1;
  }
  
  for v in result {
    println!("{}", v);
  }
}