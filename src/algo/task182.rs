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

fn helper(mut a:usize) -> Vec<bool> {
  let mut result = vec![false;20];
  for i in (0..20).rev() {
    let v = 2usize.pow(i as u32);
    if v <= a {
      a -= v;
      result[i] = true;
    }
  }
  result
}

fn main() {
  let vals: Vec<usize> = readvec();
  let (a, n) = (vals[0], vals[1]);
  let ar = helper(a);
  let nr = helper(2usize.pow(n as u32));

  let mut result = 0;
  for i in 0..ar.len() {
    if ar[i] ^ nr[i] {
      result += 2usize.pow(i as u32);
    }
  }
  println!("{}", result);
}