/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
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

fn helper(s:&[char]) -> bool {
  let n = s.len();
  let hi = n / 2;
  for i in 0..hi {
    if s[i] != s[n-1-i] {
      return false
    }
  }
  true
}

fn main() {
  input! {
    n:usize,
    k:usize,
    mut s:Chars
  }

  let mut count = 0;
  let mut seen = HashSet::new();
  let ti = n - k;
  s.sort();

  loop {
    let next: Vec<char> = s.iter().map(|v| *v).collect();

    if !seen.contains(&next) {
      let mut success = true;
      for i in 0..=ti {
        let ss = &next[i..i+k];
  
        if helper(ss) {
          success = false;
          break
        }
      }
      if success {
        count += 1;
      }
      seen.insert(next);
    }
    
    if !s.next_permutation() {
      break
    }
  } 

  println!("{}", count);

}