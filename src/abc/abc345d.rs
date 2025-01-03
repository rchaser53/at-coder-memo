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

fn debug(arr:&Vec<Vec<bool>>) {
  let n = arr.len();
  let m = arr[0].len();
  for i in 0..n {
    let mut a = format!("");
    for j in 0..m {
      if arr[i][j] {
        a = format!("{}#", a, );
      } else {
        a = format!("{}.", a, );
      }
    }

    println!("{}", a);
  }
  println!("");
}

fn main() {
  input! {
    n:usize,
    h:usize,
    w:usize,
    ab:[(usize,usize);n]
  }
  
  let mut arr = (0..n).into_iter().collect::<Vec<usize>>();
  let goal = h * w;
  loop {
    let next: Vec<usize> = arr.iter().map(|v| *v).collect();

    let border = 1 << n;
    for i in 0..border {
      let mut memo = vec![vec![false;w];h];
      let mut count = 0;
      let mut arr = vec![(0,0);n];
      for j in 0..n {
        let ri = next[j];
        if i >> j & 1 == 1 {
          arr[j] = (ab[ri].0, ab[ri].1);
        } else {
          arr[j] = (ab[ri].1, ab[ri].0);
        }
      }

      let mut now = 0;
      for ii in 0..h {
        for jj in 0..w {
          if n <= now { break }
          if memo[ii][jj] { continue }

          let (ah,aw) = arr[now];
          let li = ii+ah;
          let lj = jj+aw;
          if h < li || w < lj { continue }

          let mut success = true;
          for iii in ii..li {
            for jjj in jj..lj {
              if memo[iii][jjj] {
                success = false;
                break
              }
            }
          }
          if success {
            for iii in ii..li {
              for jjj in jj..lj {
                memo[iii][jjj] = true;
                count += 1;
              }
            }
            now += 1;
          }
        }
      }

      if count == goal {
        println!("Yes");
        return
      }
    }

    if !arr.next_permutation() {
      println!("No");
      return
    } 
  } 
}