/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

pub trait LexicalPermutation {
  fn next_permutation(&mut self) -> bool;
  fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
  fn next_permutation(&mut self) -> bool {
    if self.len() < 2 { return false; }
      let mut i = self.len() - 1;
      while i > 0 && self[i-1] >= self[i] {
        i -= 1;
      }

      if i == 0 {
        return false;
      }

      let mut j = self.len() - 1;
      while j >= i && self[j] <= self[i-1]  {
        j -= 1;
      }

      self.swap(j, i-1);

      self[i..].reverse();

      true
    }

  fn prev_permutation(&mut self) -> bool {
    if self.len() < 2 { return false; }

    let mut i = self.len() - 1;
    while i > 0 && self[i-1] <= self[i] {
      i -= 1;
    }

    if i == 0 {
      return false;
    }

    self[i..].reverse();

    let mut j = self.len() - 1;
    while j >= i && self[j-1] < self[i-1]  {
      j -= 1;
    }

    self.swap(i-1, j);

    true
  }
}

struct Helper {
    pattern: HashSet<Vec<usize>>,
}

impl Helper {
  fn dfs(&mut self, arr: &mut Vec<usize>, i:usize, mut left:usize) {
    if arr.len() <= i {
      return
    }

    if left == 0 {
      self.pattern.insert(arr.clone());
      return
    }

    self.dfs(arr, i+1, left);
    arr[i] += 1;
    left -= 1;
    self.dfs(arr, i, left);
    self.dfs(arr, i+1, left);
    arr[i] -= 1;
    left += 1;
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
    s:[String;n],
    t:[String;m]
  }

  let set = t.into_iter().collect::<HashSet<String>>();
  if n == 1 {
    if set.contains(&s[0]) || s[0].len() < 3 {
      println!("-1");
    } else {
      println!("{}", s[0]);
    }
    return
  }

  let mut tot = 0;
  for v in s.iter() {
    tot += v.len();
  }

  let diff = 16 - tot - (n-1);
  let mut helper = Helper { pattern: HashSet::new() };
  let mut dict = vec![];
  for i in 0..=diff {
    let mut arr = vec![1;n-1];
    helper.dfs(&mut arr, 0, i);
  }
  for i in 0..=16 {
    dict.push(vec![format!("_");i].into_iter().collect::<String>());
  }

  let mut pattern = helper.pattern.into_iter().collect::<Vec<Vec<usize>>>();
  pattern.sort();
  let mut indexes = (0..n).into_iter().collect::<Vec<usize>>();
  loop {
    let next: Vec<usize> = indexes.iter().map(|v| *v).collect();
    let mut base_arr = vec![""; n + n - 1];
    for i in 0..n {
      base_arr[2*i] = &s[next[i]];
    }
    for arr in &pattern {
      for i in 0..n-1 {
        base_arr[2*i+1] = &dict[arr[i]];
      }
      let temp = base_arr.join("");
      if !set.contains(&temp) {
        println!("{}", temp);
        return
      }
    }

    if !indexes.next_permutation() {
      break
    }
  } 
  println!("-1");
}