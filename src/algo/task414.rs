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
  let (n,m) = (a[0],a[1]);
  let mut g = vec![vec![];n];
  for _ in 0..m {
    let a:Vec<usize> = readvec();
    let (n,m) = (a[0],a[1]);
    g[n].push(m);
    g[m].push(n);
  }

  let mut set = HashSet::new();
  let mut stack = vec![0];
  set.insert(0);
  println!("0");
  let mut i = 0;
  while !stack.is_empty() || i < n-1 {
    let mut new_stack = vec![];
    while let Some(ci) = stack.pop() {
      for &ni in &g[ci] {
        if set.contains(&ni) { continue }
        set.insert(ni);
        new_stack.push(ni);
      }
    }
    new_stack.sort();
    println!("{}", new_stack.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
    stack = new_stack;
    i += 1;
  }
  
}