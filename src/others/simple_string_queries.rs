use either::*;
use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
  input! {
    n: usize,
    mut s: Chars,
    q: usize
  }
  
  let mut queries: Vec<(usize, usize, Either<usize, char>)> = vec![];
  for i in 0..q {
    input! { k: usize }
    if k == 1 {
      input! {
        i: Usize1,
        c: char
      }
      queries.push((k, i, Right(c)));
    } else {
      input! {
        l: Usize1,
        r: Usize1
      }
      queries.push((k, l, Left(r)));
    }
  }
  
  let mut bits: Vec<BIT<isize>> = vec![BIT::new(n); 26];
  for i in 0u8..26 {
    let c = ('a' as u8 + i) as char;
    for (ii, ns) in s.iter().enumerate() {
      if ns == &c {
        let index = (*ns as u8 - 'a' as u8) as usize;
        bits[index].add(ii,1);
      }
    }
  }
  
  for &query in &queries {
    if query.0 == 1 {
      update_bits(&mut s, &mut bits, query.1, query.2.right().unwrap());
    } else {
      let ans = calc(&bits, query.1, query.2.left().unwrap());
      println!("{}", ans);
    }
  }
}

fn update_bits(
  s: &mut Vec<char>,
  bits: &mut Vec<BIT<isize>>,
  from: usize,
  to: char
) {
  let c = (s[from] as u8 - 'a' as u8) as usize;
  bits[c].add(from, -1);
  let index = (to as u8 - 'a' as u8) as usize;
  bits[index].add(from, 1);
  s[from] = to;
}

fn calc(
  bits: &Vec<BIT<isize>>,
  l: usize,
  r: usize
) -> usize {
  let mut count = 0;
  for bit in bits {
    if bit.sum(l as isize, (r+1) as isize) != 0 {
      count += 1;
    }
  }
  count
}

use num::Num;

#[derive(Clone)]
struct BIT<T> {
  pub n: usize,
  pub d: Vec<T>
}

impl<T> BIT<T>
where T: Num + Copy + Clone {
  fn new(n: usize) -> Self {
    BIT {
      n,
      d: vec![T::zero(); n+1]
    }
  }
  
  fn add(&mut self, mut i:usize, x:T) {
    i += 1;
    while i <= self.n {
      self.d[i] = self.d[i] + x;
      i += i & i.wrapping_neg();
    }
  }
  
  fn sum_one(&self, mut i:isize) -> T {
    if i < 0 {
      return T::zero();
    }
    let mut x = T::zero();
    i += 1;
    while i != 0 {
      x = x + self.d[i as usize];
      i -= i & i.wrapping_neg();
    }
    x
  }
  
  fn sum(&self, l:isize, r:isize) -> T {
    self.sum_one(r-1) - self.sum_one(l-1)
  }
}

#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use proconio::{input, marker::*, fastout};

fn main() {
  input! {
    n: usize,
    mut s: Bytes,
    q: usize
  }
  s.iter_mut().for_each(|c| *c -= b'a');
  let mut index = vec![BTreeSet::new(); 26];
  for (i, &b) in s.iter().enumerate() {
    index[b as usize].insert(i);
  }
  
  for _ in 0..q {
    input!{ command: usize }
    if command == 1 {
      input!{ i:Usize1, c: char }
      let c = c as u8 - b'a';
      let c2 = s[i];
      s[i] = c;
      index[c2 as usize].remove(&i);
      index[c as usize].insert(i);
    } else {
      input!{ l:Usize1, r:Usize1 }
      let mut res = 0;
      for c in 0..26 {
        if index[c].range(l..=r).next().is_some() {
          dbg!(&index[c].range(l..=r), c);
          res += 1;
        }
      }
      println!("{}", res);
    }
  }
}
