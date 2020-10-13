#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

const IMOD:isize = 1_000_000_007;
const MAX:usize = 100_500;
#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    mut vals: [isize;n]
  }
  vals.sort();

  let com = COM::new(MAX, IMOD);
  let mut max = 0;
  let mut min = 0;
  for i in 0..n {
    min = (min + (com.com(n - i - 1, k - 1) * vals[i]) % IMOD) % IMOD;
    max = (max + (com.com(i, k - 1) * vals[i]) % IMOD) % IMOD;
  }

  println!("{}", (max + IMOD - min) % IMOD);
}

struct COM {
  fac: Vec<isize>,
  finv: Vec<isize>,
  inv: Vec<isize>,
  max: usize,
  m: isize,
}
 
impl COM {
  fn new(max: usize, m: isize) -> Self {
    let mut com = COM {
      fac: vec![0; max],
      finv: vec![0; max],
      inv: vec![0; max],
      max,
      m,
    };
    com.init();
    com
  }

  fn init(&mut self) {
    self.fac[0] = 1;
    self.fac[1] = 1;
    self.finv[0] = 1;
    self.finv[1] = 1;
    self.inv[0] = 0;
    self.inv[1] = 1;

    let mut i = 2;
    while i < self.max {
      self.fac[i] = self.fac[i - 1] * i as isize % self.m;
      self.inv[i] =
        self.m as isize - self.inv[self.m as usize % i] * (self.m / i as isize) % self.m;
      self.finv[i] = self.finv[i - 1] * self.inv[i] % self.m;
      i += 1;
    }
  }
 
  fn com(&self, n: usize, k: usize) -> isize {
    if n < k {
      return 0;
    }
    self.fac[n] * (self.finv[k] * self.finv[n - k] % self.m) % self.m
  }
}