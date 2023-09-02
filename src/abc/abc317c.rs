/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

pub trait LexicalPermutation {
  fn next_permutation(&mut self) -> bool;
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
}

fn main() {
  input! {
    n:usize,
    m:usize,
    abc:[(Usize1,Usize1,usize);m]
  }

  let default_val = 10usize.pow(9);
  let mut g = vec![vec![default_val;n];n];
  for (a,b,c) in abc {
    g[a][b] = c;
    g[b][a] = c;
  }

  let mut p = (0..n).into_iter().collect::<Vec<usize>>();
  let mut result = 0;

  loop {
    let temp_points: Vec<usize> = p.iter().map(|v| *v).collect();
    
    let mut temp = 0;
    for i in 0..n-1 {
      let ci = temp_points[i];
      let ni = temp_points[i+1];
      let v = g[ci][ni];
      if default_val == v {
        break
      }
      temp += v;
    }
    
    result = result.max(temp);
    if !p.next_permutation() {
      println!("{}", result);
      return
    }
  }
}