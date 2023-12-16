/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
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

fn helper(mut a:Vec<usize>) -> Vec<Vec<usize>> {
  let mut result = vec![];
  loop {
    let next: Vec<usize> = a.iter().map(|v| *v).collect();
    result.push(next);
    if !a.next_permutation() {
      break
    }
  } 

  result
}

fn helper2(p: &Vec<Vec<usize>>, a: Vec<usize>) -> Vec<usize> {
  let n = p.len();
  let m = a.len();

  let mut result = vec![0;n];
  for i in 0..n {
    let mut b = p[i].clone();
    let mut temp = 0;
    for j in 0..m {
      if a[j] != b[j] {
        for k in j+1..m {
          if a[j] == b[k] {
            for x in (j+1..=k).rev() {
              b.swap(x-1,x);
              temp += 1;
            }
            break
          }
        }
      }
    }
    result[i] = temp;
  }

  result
}

fn main() {
  input! {
    h:usize,
    w:usize,
    a:[[usize;w];h],
    b:[[usize;w];h]
  }

  let hp = helper((0..h).into_iter().collect::<Vec<usize>>());
  let wp = helper((0..w).into_iter().collect::<Vec<usize>>());

  let h_memo = helper2(&hp, (0..h).into_iter().collect::<Vec<usize>>());
  let w_memo = helper2(&wp, (0..w).into_iter().collect::<Vec<usize>>());
  let mut result = 1000;
  for i in 0..hp.len() {
    let hrp = &hp[i];
    for j in 0..wp.len() {
      let wrp = &wp[j];

      let mut success = true;
      for hi in 0..h {
        for wj in 0..w {
          if a[hrp[hi]][wrp[wj]] != b[hi][wj] {
            success = false;
            break
          }
        }
      }

      if success {
        result = result.min(h_memo[i]+w_memo[j]);
      }
    }
  }

  if result == 1000 {
    println!("-1");
  } else {
    println!("{}", result);
  }
  
}