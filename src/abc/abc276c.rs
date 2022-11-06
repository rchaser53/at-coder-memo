/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
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
  input! {
    n:usize,
    mut vals:[usize;n]
  }

  vals.prev_permutation();
  println!("{}", vals.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}