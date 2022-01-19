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
  let n = a[0];
  let mut vals = vec![];
  
  for i in 0..n {
    let base:Vec<String> = readvec();
    let s = base[0].clone();
    let a:usize = base[1].parse::<usize>().unwrap();
    let b:usize = base[2].parse::<usize>().unwrap();
    vals.push((i,s,a,b));
  }

  vals.sort_by(|a,b| {
    let v = a.2.cmp(&b.2);
    if v == Ordering::Equal {
      let av = a.2 + a.3;
      let bv = b.2 + b.3;
      let v = av.cmp(&bv);
      if v == Ordering::Equal {
        a.0.cmp(&b.0)
      } else {
        v
      }
    } else {
      if v == Ordering::Less {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    }
  });

  for (_, name, a, b) in vals {
    println!("{} {} {}", name, a, b);
  }
}