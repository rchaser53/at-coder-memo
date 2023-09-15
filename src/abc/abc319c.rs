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
    a:[[usize;3];3]
  }

  let patterns = vec![
    vec![0,3,6],  // 0
    vec![0,4],    // 1
    vec![0,5,7],  // 2
    vec![1,3],    // 3
    vec![1,4,6,7],// 4
    vec![1,5],    // 5
    vec![2,3,7],  // 6
    vec![2,4],    // 7
    vec![2,5,6]   // 8
  ];

  let tot = 9*8*7*6*5*4*3*2;
  let mut p = (0..9).into_iter().collect::<Vec<usize>>();
  let mut count = 0;
  loop {
    let next: Vec<usize> = p.iter().map(|v| *v).collect();

    let mut dirty = false;
    let mut route = vec![vec![];8];
    for i in 0..9 {
      let j = next[i];
      let ri = j / 3;
      let ci = j % 3;
      let v = a[ri][ci];

      for &ti in &patterns[j] {
        route[ti].push(v);
      }
    }

    for i in 0..8 {
      if route[i][0] == route[i][1] {
        dirty = true;
      }
    }

    if dirty {
      count += 1;
    }

    if !p.next_permutation() {
      break
    }
  }

  println!("{}", (tot - count) as f64 / tot as f64);

}