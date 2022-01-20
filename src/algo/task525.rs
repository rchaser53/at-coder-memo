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

struct Helper {
  result: Vec<usize>,
  set: HashSet<usize>,
  g:Vec<Vec<usize>>,
}

impl Helper {
  fn dfs(&mut self, ci:usize, last:usize) {
    if !self.set.contains(&ci) {
      self.result.push(ci);
    } else {
      self.set.insert(ci);
    }

    for i in 0..self.g[ci].len() {
      let ni = self.g[ci][i];
      if ni == last { continue }
      self.dfs(ni, ci);
    }
  }
}

fn main() {
  let a:Vec<usize> = readvec();
  let (n) = (a[0]);
  let vals:Vec<usize> = readvec();

  let mut g = vec![vec![];n];
  for i in 0..n-1 {
    let ni = vals[i];
    g[i+1].push(ni);
    g[ni].push(i+1);
  }
  for i in 0..n {
    g[i].sort();
  }
  
  let mut helper = Helper { result:vec![], set:HashSet::new(), g };
  helper.dfs(0, 1_000_000);

  println!("{}", helper.result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}