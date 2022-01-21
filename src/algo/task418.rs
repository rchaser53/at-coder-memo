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

  let mut stack = vec![(0,0)];
  let inf = 1_000_000_000;
  let mut memo = vec![inf;n];
  memo[0] = 0;
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      let nv = cv + 1;
      for &ni in &g[ci] {
        if memo[ni] <= nv { continue }
        memo[ni] = nv; 
        new_stack.push((ni,nv));
      }
    }
    stack = new_stack;
  }
  println!("{}", memo.iter().max().unwrap());
}