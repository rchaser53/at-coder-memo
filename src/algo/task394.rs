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
  let n:usize = readln();
  let mut x:Vec<usize> = readvec();
  let m:usize = readln();
  let a:Vec<usize> = readvec();
  
  let mut set = HashSet::new();
  for &v in &x {
    set.insert(v);
  }

  for &i in &a {
    let i = i-1;
    let cv = x[i];
    if cv == 2019 { continue }
    let nv = cv + 1;
    if set.contains(&nv) { continue }
    set.remove(&cv);
    set.insert(nv);
    x[i] = nv;
  }
  
  for v in x {
    println!("{}", v);
  }
}