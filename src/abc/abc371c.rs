/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
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
    m1:usize,
    edges1:[(Usize1,Usize1);m1],
    m2:usize,
    edges2:[(Usize1,Usize1);m2],
  }

  let mut h_set = HashSet::new();
  for (mut a, mut b) in edges2 {
    if b < a {
      std::mem::swap(&mut a, &mut b);
    }
    h_set.insert((a,b));
  }

  let inf = 10usize.pow(15);
  let mut g = vec![vec![inf;n];n];
  for i in 0..n-1 {
    input! {
      a: [usize;n-1-i]
    }

    for j in 0..a.len() {
      g[i][j+i+1] = a[j];
      g[j+i+1][i] = a[j];
    }
  }

  let mut result = inf;
  let mut patterns = (0..n).into_iter().collect::<Vec<usize>>();
  loop {
    let temp_points: Vec<usize> = patterns.iter().map(|v| *v).collect();

    let mut dict = vec![0;n];
    for i in 0..n {
      let origial_i = temp_points[i];
      dict[origial_i] = i;
    }

    let mut g_set = HashSet::new();
    for &(a, b) in &edges1 {
      let mut a = dict[a];
      let mut b = dict[b];
      if b < a {
        std::mem::swap(&mut a, &mut b);
      }
      g_set.insert((a,b));
    }

    let mut temp = 0;
    for i in 0..n {
      for j in 0..n {
        let g_exist = g_set.contains(&(i,j));
        let h_exist = h_set.contains(&(i,j));
        
        if g_exist != h_exist {
          temp += g[i][j];
        }
      }
    }
    
    result = result.min(temp);
    
    if !patterns.next_permutation() {
      break
    }
  }

  println!("{}", result);

}