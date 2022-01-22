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
  let mut g:Vec<HashSet<usize>> = vec![HashSet::new();n];
  let mut memo = vec![0;n];
  for _ in 0..m {
    let a:Vec<usize> = readvec();
    let (f,s) = (a[0],a[1]);
    g[f].insert(s);
    memo[s] += 1;
  }

  let mut que = VecDeque::new();
  for i in 0..n {
    if memo[i] == 0 {
      que.push_back(i);
    }
  }

  while let Some(ci) = que.pop_front() {
    for &ni in &g[ci] {
      memo[ni] -= 1;
      if memo[ni] == 0 {
        que.push_back(ni);
      }
    }
  }
  
  for v in memo {
    if v != 0 {
      println!("No");
      return
    }
  }
  println!("Yes");
}