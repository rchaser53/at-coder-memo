/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;


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

fn main() {
  input!{
    n:usize,
    m:usize,
    rows:[Chars;n]
  }
  
  let mut g = vec![HashSet::new();n];
  for i in 0..n {
    for j in 0..n {
      if i == j { continue }
      let mut count = 0;
      for k in 0..m {
        if rows[i][k] != rows[j][k] {
          count += 1;
        }
      }

      if count == 1 {
        g[i].insert(j);
      }
    }
  }

  let mut vals = (0..n).into_iter().collect::<Vec<usize>>();
  loop {
    let next: Vec<usize> = vals.iter().map(|v| *v).collect();
    
    let mut flag = true;
    for i in 0..n-1 {
      let ci = next[i];
      let ni = next[i+1];
      if !g[ci].contains(&ni) {
        flag = false;
        break
      }
    }

    if flag {
      println!("Yes");
      return
    }

    if !vals.next_permutation() {
      break
    }
  } 

  println!("No");
}