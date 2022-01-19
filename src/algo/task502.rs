/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
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
  let (n) = (a[0]);
  let vals:Vec<usize> = readvec();
  let mut result = vec![0;n];

  for i in vals {
    result[i] += 1;
  }

  let mut vals = result.into_iter().enumerate().collect::<Vec<(usize,usize)>>();
  vals.sort_by(|a,b| {
    let v = a.1.cmp(&b.1);
    if v == Ordering::Equal {
      let v = a.0.cmp(&b.0);
      if v == Ordering::Less {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    } else {
      v
    }
  });

  vals.reverse();
  for (i, v) in vals {
    println!("{} {}", i, v);
  }
}